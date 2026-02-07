# NigPing VPS Agent Deployment

## Prerequisites

- A VPS running Linux (Ubuntu/Debian recommended).
- **Docker** and **Docker Compose** installed.
- **WireGuard** kernel module loaded (usually default on modern kernels).

## Deployment Steps

### 1. Quick Deployment (Recommended)

Copy the `deploy-vps.sh` script to your VPS and run it:

```bash
# On your VPS:
curl -O https://raw.githubusercontent.com/lzx0122/NigPing/main/vps-agent/deploy-vps.sh # (Example URL, adjust as needed)
chmod +x deploy-vps.sh
./deploy-vps.sh
```

Or just copy-paste the content of `deploy-vps.sh`.

### 2. Manual Deployment

If you prefer to set it up manually:

1.  **Create a folder**: `mkdir ~/nigping-agent && cd ~/nigping-agent`
2.  **Create `docker-compose.yml`**:
    ```yaml
    version: "3"
    services:
      vps-agent:
        image: lzx0122/nigping-vps-agent:latest
        restart: always
        network_mode: "host"
        cap_add:
          - NET_ADMIN
        volumes:
          - /etc/wireguard:/etc/wireguard
        environment:
          - SUPABASE_URL=your_url
          - SUPABASE_SERVICE_ROLE_KEY=your_key
          - PUBLIC_IP=your_ip
    ```
3.  **Create `.env` file** with your credentials.
4.  **Start**: `docker-compose up -d`

## Does this install WireGuard?

**Yes.** The agent container handles the WireGuard interface configuration.

- **Agent**: Runs in a Docker container.
- **WireGuard**: The agent uses the host's kernel WireGuard module (via `network_mode: "host"`).
  - _Note_: Ensure your VPS OS supports WireGuard (most modern Ubuntu/Debian/CentOS do).
  - If `wg-quick` fails, try installing wireguard tools on the host: `apt install wireguard-tools`.
