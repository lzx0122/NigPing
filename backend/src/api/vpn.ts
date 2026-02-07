import { Hono } from "hono";
import { supabase } from "../db/supabase.js";

const app = new Hono();

// Register new device (no server selection)
app.post("/register", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;

    const { publicKey, deviceName } = await c.req.json<{
      publicKey: string;
      deviceName: string;
    }>();

    if (!publicKey || !deviceName) {
      return c.json({ error: "Missing required fields" }, 400);
    }

    // Check device count limit (max 5 devices per user)
    const { count, error: countError } = await supabase
      .from("vpn_profiles")
      .select("*", { count: "exact", head: true })
      .eq("user_id", userId);

    if (countError) {
      console.error("Count Error:", countError);
      return c.json({ error: "Failed to check device count" }, 500);
    }

    if (count !== null && count >= 5) {
      return c.json(
        {
          error:
            "Maximum device limit reached (5 devices). Please delete an existing device before registering a new one.",
        },
        400,
      );
    }

    // Call RPC to register device
    const { data, error } = await supabase.rpc("register_vpn_device", {
      p_user_id: userId,
      p_device_name: deviceName,
      p_public_key: publicKey,
    });

    if (error) {
      console.error("RPC Error:", error);
      return c.json({ error: error.message }, 500);
    }

    if (data && data.success === false) {
      return c.json({ error: data.error }, 400);
    }

    return c.json(data);
  } catch (e) {
    console.error("VPN Register Error:", e);
    return c.json({ error: "Internal Server Error" }, 500);
  }
});

// Connect to specific server (allocates IP if needed)
app.post("/connect", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;

    const { profileId, serverIp } = await c.req.json<{
      profileId: string;
      serverIp: string;
    }>();

    if (!profileId || !serverIp) {
      return c.json({ error: "Missing required fields" }, 400);
    }

    // Verify profile belongs to user
    const { data: profile, error: profileError } = await supabase
      .from("vpn_profiles")
      .select("id")
      .eq("id", profileId)
      .eq("user_id", userId)
      .single();

    if (profileError || !profile) {
      return c.json({ error: "Profile not found" }, 404);
    }

    // Call RPC to allocate IP for this server
    const { data, error } = await supabase.rpc("allocate_server_ip", {
      p_profile_id: profileId,
      p_server_ip: serverIp,
    });

    if (error) {
      console.error("RPC Error:", error);
      return c.json({ error: error.message }, 500);
    }

    if (data && data.success === false) {
      return c.json({ error: data.error }, 400);
    }

    return c.json(data);
  } catch (e) {
    console.error("VPN Connect Error:", e);
    return c.json({ error: "Internal Server Error" }, 500);
  }
});

// Get user profiles (devices)
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

// Get allocations for a specific profile
app.get("/profiles/:id/allocations", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;
    const profileId = c.req.param("id");

    // Verify profile belongs to user
    const { data: profile, error: profileError } = await supabase
      .from("vpn_profiles")
      .select("id")
      .eq("id", profileId)
      .eq("user_id", userId)
      .single();

    if (profileError || !profile) {
      return c.json({ error: "Profile not found" }, 404);
    }

    // Get allocations
    const { data, error } = await supabase
      .from("vpn_server_allocations")
      .select("*")
      .eq("profile_id", profileId)
      .order("created_at", { ascending: false });

    if (error) throw error;

    return c.json(data);
  } catch (e) {
    console.error("Fetch Allocations Error:", e);
    return c.json({ error: "Failed to fetch allocations" }, 500);
  }
});

// Delete profile (device)
app.delete("/profiles/:id", async (c) => {
  try {
    const jwtPayload = c.get("jwtPayload");
    const userId = jwtPayload.sub;
    const profileId = c.req.param("id");

    const { error } = await supabase
      .from("vpn_profiles")
      .delete()
      .eq("id", profileId)
      .eq("user_id", userId);

    if (error) throw error;

    return c.json({ success: true });
  } catch (e) {
    console.error("Delete Profile Error:", e);
    return c.json({ error: "Failed to delete profile" }, 500);
  }
});

export default app;
