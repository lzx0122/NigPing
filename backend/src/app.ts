import { Hono } from "hono";
import { cors } from "hono/cors";
import { readFileSync, existsSync } from "fs";
import { join } from "path";
import { supabase } from "./db/supabase";

const app = new Hono();

// Enable CORS for frontend development
app.use("/*", cors());

app.get("/", (c) => {
  return c.text("NigPing Backend is running!");
});

// Get all servers
app.get("/api/servers", async (c) => {
  try {
    const { data, error } = await supabase
      .from("servers")
      .select("*")
      .order("added_at", { ascending: false });

    if (error) throw error;

    // Transform to match frontend expectations
    const servers = (data || []).map((server: any) => ({
      ip: server.ip,
      region: server.region,
      addedAt: server.added_at,
    }));

    return c.json(servers);
  } catch (error) {
    console.error("Error fetching servers:", error);
    return c.json({ error: "Failed to fetch servers" }, 500);
  }
});

// Add new server
app.post("/api/servers", async (c) => {
  try {
    const body = await c.req.json<{ ip: string; region: string }>();

    if (!body.ip || !body.region) {
      return c.json({ error: "Missing ip or region" }, 400);
    }

    const { data, error } = await supabase
      .from("servers")
      .insert([
        {
          ip: body.ip,
          region: body.region,
        },
      ])
      .select()
      .single();

    if (error) {
      // Check for unique constraint violation
      if (error.code === "23505") {
        return c.json({ error: "Server with this IP already exists" }, 409);
      }
      throw error;
    }

    // Transform to match frontend expectations
    const newServer = {
      ip: data.ip,
      region: data.region,
      addedAt: data.added_at,
    };

    return c.json(newServer, 201);
  } catch (error) {
    console.error("Error adding server:", error);
    return c.json({ error: "Failed to add server" }, 500);
  }
});

// Update server
app.put("/api/servers/:ip", async (c) => {
  try {
    const oldIp = c.req.param("ip");
    const body = await c.req.json<{ ip: string; region: string }>();

    if (!body.ip || !body.region) {
      return c.json({ error: "Missing ip or region" }, 400);
    }

    const { data, error } = await supabase
      .from("servers")
      .update({
        ip: body.ip,
        region: body.region,
        updated_at: new Date().toISOString(),
      })
      .eq("ip", oldIp)
      .select()
      .single();

    if (error) {
      if (error.code === "PGRST116") {
        return c.json({ error: "Server not found" }, 404);
      }
      throw error;
    }

    // Transform to match frontend expectations
    const updatedServer = {
      ip: data.ip,
      region: data.region,
      addedAt: data.added_at,
    };

    return c.json(updatedServer);
  } catch (error) {
    console.error("Error updating server:", error);
    return c.json({ error: "Failed to update server" }, 500);
  }
});

// Delete server
app.delete("/api/servers/:ip", async (c) => {
  try {
    const ip = c.req.param("ip");

    const { error } = await supabase.from("servers").delete().eq("ip", ip);

    if (error) throw error;

    return c.json({ success: true });
  } catch (error) {
    console.error("Error deleting server:", error);
    return c.json({ error: "Failed to delete server" }, 500);
  }
});

// === Game IP Ranges Endpoints ===

// Get IP ranges for a specific game
app.get("/api/games/:gameId/ranges", async (c) => {
  try {
    const gameId = c.req.param("gameId");
    const { data, error } = await supabase
      .from("game_ip_ranges")
      .select("ip_range")
      .eq("game_id", gameId);

    if (error) throw error;

    // Return array of strings
    return c.json((data || []).map((row: any) => row.ip_range));
  } catch (error) {
    console.error("Error fetching game IP ranges:", error);
    return c.json({ error: "Failed to fetch game IP ranges" }, 500);
  }
});

// Add new IP range for a specific game
app.post("/api/games/:gameId/ranges", async (c) => {
  try {
    const gameId = c.req.param("gameId");
    const body = await c.req.json<{ ipRange: string }>();

    if (!body.ipRange) {
      return c.json({ error: "Missing ipRange" }, 400);
    }

    const { data, error } = await supabase
      .from("game_ip_ranges")
      .insert([
        {
          game_id: gameId,
          ip_range: body.ipRange,
        },
      ])
      .select()
      .single();

    if (error) {
      // If unique constraint violation, just return success (idempotent)
      if (error.code === "23505") {
        return c.json({ message: "Range already exists" }, 200);
      }
      throw error;
    }

    return c.json({ success: true, range: data }, 201);
  } catch (error) {
    console.error("Error adding game IP range:", error);
    return c.json({ error: "Failed to add game IP range" }, 500);
  }
});

// Delete IP range for a specific game
app.delete("/api/games/:gameId/ranges", async (c) => {
  try {
    const gameId = c.req.param("gameId");
    const ipRange = c.req.query("range");

    if (!ipRange) {
      return c.json({ error: "Missing range query parameter" }, 400);
    }

    const { error } = await supabase
      .from("game_ip_ranges")
      .delete()
      .eq("game_id", gameId)
      .eq("ip_range", ipRange);

    if (error) throw error;

    return c.json({ success: true });
  } catch (error) {
    console.error("Error deleting game IP range:", error);
    return c.json({ error: "Failed to delete game IP range" }, 500);
  }
});

// Get stats
app.get("/api/stats", async (c) => {
  try {
    const { data, error } = await supabase.from("servers").select("region");

    if (error) throw error;

    const byRegion: Record<string, number> = {};
    (data || []).forEach((server: any) => {
      byRegion[server.region] = (byRegion[server.region] || 0) + 1;
    });

    return c.json({
      total: data?.length || 0,
      byRegion,
    });
  } catch (error) {
    console.error("Error fetching stats:", error);
    return c.json({ error: "Failed to fetch stats" }, 500);
  }
});

// Serve static assets (CSS, JS) from admin-ui/dist/assets
app.get("/assets/*", (c) => {
  const assetPath = c.req.path.replace("/assets/", "");
  const fullPath = join(process.cwd(), "admin-ui", "dist", "assets", assetPath);

  if (existsSync(fullPath)) {
    const content = readFileSync(fullPath);

    // Set appropriate content type
    const ext = assetPath.split(".").pop()?.toLowerCase();
    const contentTypes: Record<string, string> = {
      js: "application/javascript",
      css: "text/css",
      svg: "image/svg+xml",
      png: "image/png",
      jpg: "image/jpeg",
      jpeg: "image/jpeg",
    };

    const contentType = contentTypes[ext || ""] || "application/octet-stream";
    return c.body(content, 200, { "Content-Type": contentType });
  }

  return c.text("Asset not found", 404);
});

// Serve vite.svg from admin-ui/dist
app.get("/vite.svg", (c) => {
  const svgPath = join(process.cwd(), "admin-ui", "dist", "vite.svg");
  if (existsSync(svgPath)) {
    const content = readFileSync(svgPath);
    return c.body(content, 200, { "Content-Type": "image/svg+xml" });
  }
  return c.text("Not found", 404);
});

// Serve static files from admin-ui/dist (production build)
app.get("/admin", (c) => {
  const indexPath = join(process.cwd(), "admin-ui", "dist", "index.html");
  if (existsSync(indexPath)) {
    const html = readFileSync(indexPath, "utf-8");
    return c.html(html);
  }
  return c.text("Admin UI not built. Run 'pnpm run admin:build' first.", 404);
});

export default app;
