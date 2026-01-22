import { serve } from "@hono/node-server";
import { Hono } from "hono";
import { cors } from "hono/cors";

const app = new Hono();

// Enable CORS for frontend development
app.use("/*", cors());

// Simple in-memory storage for prototype
interface Server {
  ip: string;
  region: string;
  addedAt: string;
}

const servers: Server[] = [];

app.get("/", (c) => {
  return c.text("NigPing Backend is running!");
});

app.get("/api/servers", (c) => {
  return c.json(servers);
});

app.post("/api/servers", async (c) => {
  try {
    const body = await c.req.json<{ ip: string; region: string }>();

    if (!body.ip || !body.region) {
      return c.json({ error: "Missing ip or region" }, 400);
    }

    const newServer: Server = {
      ip: body.ip,
      region: body.region,
      addedAt: new Date().toISOString(),
    };

    servers.push(newServer);
    return c.json(newServer, 201);
  } catch (e) {
    return c.json({ error: "Invalid JSON" }, 400);
  }
});

// Serve Admin UI
app.get("/admin", (c) => {
  return c.html(htmlContent);
});

const htmlContent = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NigPing Server Manager</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; background: #f4f4f5; }
        .container { background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
        h1 { margin-top: 0; color: #18181b; }
        .form-group { margin-bottom: 1rem; }
        label { display: block; margin-bottom: 0.5rem; color: #3f3f46; font-weight: 500; }
        input { width: 100%; padding: 0.5rem; border: 1px solid #d4d4d8; border-radius: 4px; box-sizing: border-box; }
        button { background: #18181b; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; font-weight: 500; }
        button:hover { background: #27272a; }
        table { width: 100%; border-collapse: collapse; margin-top: 2rem; }
        th, td { text-align: left; padding: 0.75rem; border-bottom: 1px solid #e4e4e7; }
        th { font-weight: 600; color: #71717a; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Server Management</h1>
        <div style="display: flex; gap: 1rem; margin-bottom: 1rem;">
            <div style="flex: 1;">
                <label>IP Address</label>
                <input type="text" id="ipInput" placeholder="1.2.3.4">
            </div>
            <div style="flex: 1;">
                <label>Region</label>
                <input type="text" id="regionInput" placeholder="Taiwan">
            </div>
            <div style="display: flex; align-items: flex-end;">
                <button onclick="addServer()">Add Server</button>
            </div>
        </div>
        
        <table>
            <thead>
                <tr>
                    <th>IP Address</th>
                    <th>Region</th>
                    <th>Added At</th>
                </tr>
            </thead>
            <tbody id="serverList"></tbody>
        </table>
    </div>

    <script>
        async function loadServers() {
            const res = await fetch('/api/servers');
            const servers = await res.json();
            const tbody = document.getElementById('serverList');
            tbody.innerHTML = servers.map(s => \`
                <tr>
                    <td>\${s.ip}</td>
                    <td>\${s.region}</td>
                    <td>\${new Date(s.addedAt).toLocaleString()}</td>
                </tr>
            \`).join('');
        }

        async function addServer() {
            const ip = document.getElementById('ipInput').value;
            const region = document.getElementById('regionInput').value;
            if(!ip || !region) return alert('Please fill in all fields');

            const res = await fetch('/api/servers', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ ip, region })
            });

            if(res.ok) {
                document.getElementById('ipInput').value = '';
                document.getElementById('regionInput').value = '';
                loadServers();
            } else {
                alert('Failed to add server');
            }
        }

        loadServers();
    </script>
</body>
</html>
`;

const port = 3000;
console.log(`Server is running on port ${port}`);

serve({
  fetch: app.fetch,
  port,
});
