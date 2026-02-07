import { Hono } from "hono";
import { supabase } from "../db/supabase.js";

const app = new Hono();

// Register new VPN profile
app.post("/register", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;

    const { publicKey, deviceName, serverIp } = await c.req.json<{
      publicKey: string;
      deviceName: string;
      serverIp: string;
    }>();

    if (!publicKey || !deviceName || !serverIp) {
      return c.json({ error: "Missing required fields" }, 400);
    }

    // Call RPC
    const { data, error } = await supabase.rpc("allocate_vpn_ip", {
      p_user_id: userId,
      p_server_ip: serverIp,
      p_device_name: deviceName,
      p_public_key: publicKey,
    });

    if (error) {
      console.error("RPC Error:", error);
      return c.json({ error: error.message }, 500);
    }

    // Check application-level error returned by RPC
    if (data && data.success === false) {
      return c.json({ error: data.error }, 400);
    }

    return c.json(data);
  } catch (e) {
    console.error("VPN Register Error:", e);
    return c.json({ error: "Internal Server Error" }, 500);
  }
});

// Get user profiles
app.get("/profiles", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;

    const { data, error } = await supabase
      .from("vpn_profiles")
      .select("*")
      .eq("user_id", userId)
      .order("created_at", { ascending: false });

    if (error) throw error;

    return c.json(data);
  } catch (e) {
    console.error("Fetch Profiles Error:", e);
    return c.json({ error: "Failed to fetch profiles" }, 500);
  }
});

// Delete profile
app.delete("/profiles/:id", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;
    const profileId = c.req.param("id");

    const { error } = await supabase
      .from("vpn_profiles")
      .delete()
      .eq("id", profileId)
      .eq("user_id", userId); // Ensure user owns the profile

    if (error) throw error;

    return c.json({ success: true });
  } catch (e) {
    console.error("Delete Profile Error:", e);
    return c.json({ error: "Failed to delete profile" }, 500);
  }
});

export default app;
