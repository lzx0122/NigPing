<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Button } from "@/components/ui/button";
import { useVpnProfile } from "@/composables/useVpnProfile";
import { Loader2, Monitor, Server } from "lucide-vue-next";

const { servers, fetchServers, registerProfile, generateKeys } =
  useVpnProfile();

const deviceName = ref("");
const selectedServerIp = ref("");
const isRegistering = ref(false);
const error = ref<string | null>(null);

const emit = defineEmits(["profile-registered"]);

onMounted(() => {
  fetchServers();
});

async function handleRegister() {
  if (!deviceName.value || !selectedServerIp.value) {
    error.value = "Please fill in all fields.";
    return;
  }

  isRegistering.value = true;
  error.value = null;

  try {
    // 1. Generate WireGuard Keys
    const keys = generateKeys();
    console.log("Keys generated:", keys.publicKey);

    // 2. Call API
    const result = await registerProfile(
      deviceName.value,
      selectedServerIp.value,
      keys.privateKey,
      keys.publicKey,
    );

    console.log("Registration success:", result);

    // 3. Save Private Key locally (crucial!)
    // For MVP, we save to localStorage with a prefix.
    // Ideally use tauri-plugin-store or safe storage.
    const profileId = "default"; // Simplified for single profile per device context
    const storageKey = `vpn_profile_${profileId}`;

    // Construct full config structure to save
    const vpnConfig = {
      privateKey: keys.privateKey,
      address: result.assigned_ip, // e.g., 10.0.0.5/32 (actually usually we want /24 or /32)
      // The API returns just the IP usually "10.0.0.5".
      // We might need to append /32 or use /24 if that's the topology.
      // Plan said: assigned_ip

      serverPublicKey: result.server_public_key,
      serverEndpoint: result.server_endpoint,
      allowedIps: result.allowed_ips || "0.0.0.0/0",
      dns: "1.1.1.1", // Hardcoded or returned
    };

    localStorage.setItem(storageKey, JSON.stringify(vpnConfig));

    emit("profile-registered", vpnConfig);
  } catch (e: any) {
    console.error(e);
    error.value = e.message || "Registration failed";
  } finally {
    isRegistering.value = false;
  }
}
</script>

<template>
  <div
    class="p-6 bg-zinc-900 border border-zinc-800 rounded-xl max-w-md mx-auto"
  >
    <div class="flex items-center gap-3 mb-6">
      <div
        class="w-10 h-10 rounded-full bg-white/10 flex items-center justify-center"
      >
        <Monitor class="w-5 h-5 text-white" />
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Device Registration</h2>
        <p class="text-xs text-zinc-400">Register this device to access VPN</p>
      </div>
    </div>

    <div class="space-y-4">
      <!-- Device Name Input -->
      <div class="space-y-2">
        <label class="text-xs uppercase font-bold text-zinc-500 tracking-wider"
          >Device Name</label
        >
        <input
          v-model="deviceName"
          type="text"
          placeholder="e.g. My Gaming PC"
          class="w-full bg-black border border-zinc-800 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-white transition-colors"
        />
      </div>

      <!-- Server Selection -->
      <div class="space-y-2">
        <label class="text-xs uppercase font-bold text-zinc-500 tracking-wider"
          >Select Server</label
        >
        <div class="grid gap-2">
          <div
            v-for="server in servers"
            :key="server.ip"
            @click="selectedServerIp = server.ip"
            class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all hover:bg-zinc-800"
            :class="
              selectedServerIp === server.ip
                ? 'bg-zinc-800 border-white text-white'
                : 'bg-black border-zinc-800 text-zinc-400'
            "
          >
            <Server class="w-4 h-4" />
            <div class="flex-1">
              <div class="font-bold text-sm">{{ server.region }}</div>
              <div class="text-xs font-mono opacity-60">{{ server.ip }}</div>
            </div>
            <div
              v-if="selectedServerIp === server.ip"
              class="w-2 h-2 rounded-full bg-green-500"
            ></div>
          </div>

          <div
            v-if="servers.length === 0"
            class="text-zinc-500 text-sm italic py-2"
          >
            No servers available. Please deploy a VPS Agent first.
          </div>
        </div>
      </div>

      <!-- Error Message -->
      <div
        v-if="error"
        class="p-3 bg-red-900/20 border border-red-900 rounded-lg text-red-500 text-sm"
      >
        {{ error }}
      </div>

      <!-- Register Button -->
      <Button
        @click="handleRegister"
        :disabled="isRegistering || !deviceName || !selectedServerIp"
        class="w-full h-12 bg-white text-black hover:bg-zinc-200 font-bold"
      >
        <Loader2 v-if="isRegistering" class="w-4 h-4 mr-2 animate-spin" />
        {{ isRegistering ? "Registering..." : "Register Device" }}
      </Button>
    </div>
  </div>
</template>
