import { ref } from "vue";
import { generateKeyPair } from "@stablelib/x25519";
import { encode } from "@stablelib/base64";
import { randomBytes } from "@stablelib/random";
import { useAuth } from "./useAuth";

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
  const { getAuthHeaders } = useAuth();

  const profiles = ref<VpnProfile[]>([]);
  const servers = ref<ServerInfo[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // Generate WireGuard Key Pair (Base64 encoded)
  function generateKeys() {
    // Generate private key (32 bytes)
    const privateKeyBytes = randomBytes(32);
    // Clamp private key (standard for X25519)
    privateKeyBytes[0] &= 248;
    privateKeyBytes[31] &= 127;
    privateKeyBytes[31] |= 64;

    const keyPair = generateKeyPair(privateKeyBytes);

    return {
      privateKey: encode(keyPair.secretKey),
      publicKey: encode(keyPair.publicKey),
    };
  }

  async function fetchServers() {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/api/servers`, {
        headers: getAuthHeaders(),
      });
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
        headers: getAuthHeaders(),
      });
      if (!response.ok) throw new Error("Failed to fetch profiles");
      profiles.value = await response.json();
    } catch (e: any) {
      console.error("Fetch Profiles Error:", e);
      error.value = e.message || "Unknown error";
    } finally {
      isLoading.value = false;
    }
  }

  async function registerProfile(
    deviceName: string,
    serverIp: string,
    privateKey: string,
    publicKey: string,
  ) {
    isLoading.value = true;
    error.value = null;
    try {
      const response = await fetch(`${API_URL}/api/vpn/register`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...getAuthHeaders(),
        },
        body: JSON.stringify({
          publicKey,
          deviceName,
          serverIp,
        }),
      });

      if (!response.ok) {
        const errData = await response.json().catch(() => ({}));
        throw new Error(errData.error || "Registration failed");
      }

      const result = await response.json();

      // Refresh profiles list
      await fetchProfiles();

      // Ideally, save the private key locally securely associated with this profile ID
      // For now, we return it so the component can handle storage/config generation
      return result;
    } catch (e: any) {
      console.error("Register Profile Error:", e);
      error.value = e.message || "Unknown error";
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function connectToProfile(profile: VpnProfile, privateKey: string) {
    // Logic to configure WireGuard via Tauri command would go here or be returned
    // This helper could format the config string
  }

  return {
    profiles,
    servers,
    isLoading,
    error,
    generateKeys,
    fetchServers,
    fetchProfiles,
    registerProfile,
  };
}
