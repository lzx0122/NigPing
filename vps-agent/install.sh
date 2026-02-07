#!/bin/bash

# Check if .env exists, if not warn user
if [ ! -f .env ]; then
    echo "Warning: .env file not found. Ensure SUPABASE_URL and KEY are set in docker-compose.yml"
fi

echo "Building and Starting VPS Agent..."
docker-compose up --build -d

echo "Agent started! Logs:"
docker-compose logs -f
