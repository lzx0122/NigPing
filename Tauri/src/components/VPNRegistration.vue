<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { useVpnProfile } from "@/composables/useVpnProfile";
import { Loader2, Monitor } from "lucide-vue-next";

const { registerProfile, generateKeys } = useVpnProfile();

const deviceName = ref("");
const isRegistering = ref(false);
const error = ref<string | null>(null);
const deviceCount = ref(0);
const isLoadingDeviceName = ref(true);

const emit = defineEmits(["profile-registered"]);

onMounted(async () => {
  await fetchDeviceCount();
  await autoDetectDeviceName();
});

async function autoDetectDeviceName() {
  try {
    const name = await invoke<string>("get_device_name");
    deviceName.value = name;
  } catch (e) {
    console.error("Failed to get device name:", e);
    deviceName.value = "My Device";
  } finally {
    isLoadingDeviceName.value = false;
  }
}

async function fetchDeviceCount() {
  try {
    const token = localStorage.getItem("auth_token");
    if (!token) return;

    const response = await fetch("http://localhost:3000/api/vpn/profiles", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (response.ok) {
      const profiles = await response.json();
      deviceCount.value = profiles.length;
    }
  } catch (e) {
    console.error("Failed to fetch device count:", e);
  }
}

async function handleRegister() {
  if (!deviceName.value) {
    error.value = "Please enter a device name.";
    return;
  }

  isRegistering.value = true;
  error.value = null;

  try {
    // 1. Generate WireGuard Keys
    const keys = generateKeys();
    console.log("Keys generated:", keys.publicKey);

    // 2. Call API (no server selection needed)
    const result = await registerProfile(
      deviceName.value,
      keys.privateKey,
      keys.publicKey,
    );

    console.log("Registration success:", result);

    // 3. Save profile ID and private key locally
    const profileId = result.profile_id;
    const storageKey = `vpn_profile_${profileId}`;

    const vpnConfig = {
      profileId,
      deviceName: deviceName.value,
      privateKey: keys.privateKey,
      publicKey: keys.publicKey,
    };

    localStorage.setItem(storageKey, JSON.stringify(vpnConfig));
    localStorage.setItem("vpn_config", JSON.stringify(vpnConfig));

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
        <div class="flex items-center justify-between">
          <label
            class="text-xs uppercase font-bold text-zinc-500 tracking-wider"
            >Device Name</label
          >
          <span class="text-xs text-zinc-600">{{ deviceCount }}/5 devices</span>
        </div>
        <input
          v-model="deviceName"
          type="text"
          placeholder="e.g. My Gaming PC"
          :disabled="isLoadingDeviceName"
          class="w-full bg-black border border-zinc-800 rounded-lg px-4 py-3 text-white focus:outline-none focus:border-white transition-colors disabled:opacity-50"
        />
        <p v-if="isLoadingDeviceName" class="text-xs text-zinc-500 italic">
          Detecting device name...
        </p>
        <p class="text-xs text-zinc-400">
          ✨ You can connect to any VPS server after registration
        </p>
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
        :disabled="isRegistering || !deviceName"
        class="w-full h-12 bg-white text-black hover:bg-zinc-200 font-bold"
      >
        <Loader2 v-if="isRegistering" class="w-4 h-4 mr-2 animate-spin" />
        {{ isRegistering ? "Registering..." : "Register Device" }}
      </Button>
    </div>
  </div>
</template>
