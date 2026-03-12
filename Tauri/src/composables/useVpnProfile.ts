import { ref } from "vue";

import { useAuthStore } from "../stores/authStore";

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

export interface VpnProfile {
  id: string;
  user_id: string;
  server_ip: string;
  device_name: string;
  public_key: string;
  allowed_ip: string;
  is_active: boolean;
  created_at: string;
}

export interface ServerInfo {
  ip: string;
  region: string;
  addedAt: string;
}

export function useVpnProfile() {
  const profiles = ref<VpnProfile[]>([]);
  const servers = ref<ServerInfo[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  const authStore = useAuthStore();

  const getHeaders = () => {
    return {
      "Content-Type": "application/json",
      ...authStore.getAuthHeaders(),
    };
  };

  /**
   * Generates a new WireGuard keypair using @stablelib.
   */
  const generateKeys = async (): Promise<{
    privateKey: string;
    publicKey: string;
  }> => {
    try {
      const { generateKeyPair } = await import("@stablelib/x25519");
      const { encode } = await import("@stablelib/base64");

      const keypair = generateKeyPair();
      return {
        privateKey: encode(keypair.secretKey),
        publicKey: encode(keypair.publicKey),
      };
    } catch (e: any) {
      console.error("Failed to generate WireGuard keypair:", e);
      throw new Error("Failed to generate WireGuard keypair.");
    }
  };

  async function fetchServers() {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/api/servers`, {
        headers: getHeaders(),
      });
      if (response.status === 401) {
        authStore.logout();
        throw new Error("Session expired. Please login again.");
      }
      if (!response.ok) throw new Error("Failed to fetch servers");
      servers.value = await response.json();
    } catch (e: any) {
      console.error("Fetch Servers Error:", e);
      error.value = e.message || "Unknown error";
    } finally {
      isLoading.value = false;
    }
  }

  async function fetchProfiles() {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/api/vpn/profiles`, {
        headers: getHeaders(),
      });
      if (response.status === 401) {
        authStore.logout();
        throw new Error("Session expired. Please login again.");
      }
      if (!response.ok) throw new Error("Failed to fetch profiles");
      profiles.value = await response.json();
    } catch (e: any) {
      console.error("Fetch Profiles Error:", e);
      error.value = e.message || "Unknown error";
    } finally {
      isLoading.value = false;
    }
  }

  async function registerProfile(deviceName: string, publicKey: string) {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/api/vpn/register`, {
        method: "POST",
        headers: getHeaders(),
        body: JSON.stringify({
          publicKey,
          deviceName,
        }),
      });

      if (response.status === 401) {
        authStore.logout();
        throw new Error("Session expired. Please login again.");
      }

      if (!response.ok) {
        const errData = await response.json().catch(() => ({}));
        console.error("Registration error response:", errData);
        throw new Error(errData.error || "Registration failed");
      }

      const result = await response.json();

      // Refresh profiles list
      await fetchProfiles();

      return result;
    } catch (e: any) {
      console.error("Registration error:", e);
      error.value = e.message || "Failed to register VPN profile";
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * Requests connection to a specific server for an existing profile.
   */
  const connectToServer = async (
    profileId: string,
    serverIp: string,
    endpointPort?: number,
  ) => {
    isLoading.value = true;
    error.value = null;

    try {
      if (!authStore.isAuthenticated) {
        throw new Error("User not authenticated");
      }

      console.log(
        `[useVpnProfile] Connecting profile ${profileId} to server ${serverIp}`,
      );

      const response = await fetch(`${API_URL}/api/vpn/connect`, {
        method: "POST",
        headers: getHeaders(),
        body: JSON.stringify({
          profile_id: profileId,
          server_ip: serverIp,
          endpoint_port: endpointPort,
        }),
      });

      if (!response.ok) {
        const data = await response.json().catch(() => ({}));
        throw new Error(data.message || "Failed to connect to server");
      }

      const connectionData = await response.json();
      console.log("[useVpnProfile] Received connection data:", connectionData);

      return connectionData;
    } catch (e: any) {
      console.error("Connection request error:", e);
      error.value = e.message || "Failed to request server connection";
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  return {
    profiles,
    servers,
    isLoading,
    error,
    generateKeys,
    fetchServers,
    fetchProfiles,
    registerProfile,
    connectToServer,
  };
}
