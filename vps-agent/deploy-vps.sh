#!/bin/bash

# NigPing VPS Agent Deployment Script
# Usage: ./deploy-vps.sh

echo "========================================="
echo "   NigPing VPS Agent Deployment"
echo "========================================="

# 1. Check for Docker
if ! command -v docker &> /dev/null; then
    echo "Error: Docker could not be found."
    echo "Please install Docker and Docker Compose first."
    echo "Visit: https://docs.docker.com/engine/install/"
    exit 1
fi

# 2. Setup Directory
INSTALL_DIR=~/nigping-agent
mkdir -p $INSTALL_DIR
cd $INSTALL_DIR

echo "Installing to: $INSTALL_DIR"

# 3. Create docker-compose.yml
# We write it directly here to avoid needing to download it separately
cat <<EOF > docker-compose.yml
version: "3"
services:
  vps-agent:
    image: \${DOCKER_IMAGE_NAME:-lzx0122/nigping-vps-agent:latest}
    restart: always
    network_mode: "host"
    privileged: true
    volumes:
      - /etc/wireguard:/etc/wireguard
      - /lib/modules:/lib/modules # Map kernel modules just in case
    environment:
      - SUPABASE_URL=\${SUPABASE_URL}
      - SUPABASE_SERVICE_ROLE_KEY=\${SUPABASE_SERVICE_ROLE_KEY}
      - PUBLIC_IP=\${PUBLIC_IP}
      - PORT=\${PORT:-3000}
      - WG_INTERFACE=wg0
EOF

# 4. Configure Environment
if [ ! -f .env ]; then
    echo ""
    echo "--- Configuration ---"
    read -p "Enter Supabase URL: " SUPABASE_URL
    read -p "Enter Supabase Service Role Key: " SUPABASE_KEY
    
    # Auto-detect IP if possible
    DETECTED_IP=$(curl -s ifconfig.me || echo "")
    read -p "Enter VPS Public IP [${DETECTED_IP}]: " PUBLIC_IP
    PUBLIC_IP=${PUBLIC_IP:-$DETECTED_IP}

    cat <<EOF > .env
SUPABASE_URL=$SUPABASE_URL
SUPABASE_SERVICE_ROLE_KEY=$SUPABASE_KEY
PUBLIC_IP=$PUBLIC_IP
PORT=3000
DOCKER_IMAGE_NAME=lzx0122/nigping-vps-agent:latest
EOF
    echo ".env file created."
else
    echo ".env file already exists. Skipping configuration."
fi

# Check for docker-compose or docker compose
if command -v docker-compose &> /dev/null; then
    COMPOSE_CMD="docker-compose"
elif docker compose version &> /dev/null; then
    COMPOSE_CMD="docker compose"
else
    echo "Error: Docker Compose not found."
    exit 1
fi

echo "Starting VPS Agent with $COMPOSE_CMD..."
$COMPOSE_CMD pull
$COMPOSE_CMD up -d

echo ""
echo "========================================="
echo "   Deployment Complete!"
echo "========================================="
echo "Logs: $COMPOSE_CMD logs -f"
echo "Status: $COMPOSE_CMD ps"
