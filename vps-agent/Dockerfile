FROM node:20-alpine

# Install WireGuard tools and dependencies
RUN apk add --no-cache wireguard-tools iptables curl ip6tables

WORKDIR /app

# Install dependencies
COPY package.json pnpm-lock.yaml* ./
RUN corepack enable && corepack prepare pnpm@latest --activate
RUN pnpm install

# Build the project
COPY . .
RUN pnpm run build

# Create config creation dir
RUN mkdir -p /etc/wireguard

EXPOSE 3000

CMD ["node", "dist/index.js"]
