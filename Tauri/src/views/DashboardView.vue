<script setup lang="ts">
import { ref } from "vue";
import ServerDetection from "../components/ServerDetection.vue";
import DeviceManager from "../components/DeviceManager.vue";

// Process state mapped from App.vue
const processName = ref("ShooterGame.exe");
const detectedGameId = ref("default");
const knownRanges = ref(new Set<string>());

function onNewRangeDetected() {
  console.log("New game server range detected and added!");
}
</script>

<template>
  <div class="h-full flex flex-col p-6 space-y-6 overflow-y-auto">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-white tracking-tight">Dashboard</h1>
    </div>

    <!-- Main Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Device/Profile Management -->
      <div class="space-y-4">
        <h2 class="text-xs font-bold text-zinc-500 uppercase tracking-wider">
          Connection Settings
        </h2>
        <DeviceManager />
      </div>

      <!-- Server Detection -->
      <div class="space-y-4">
        <h2 class="text-xs font-bold text-zinc-500 uppercase tracking-wider">
          Game Monitoring
        </h2>
        <ServerDetection
          :process-name="processName"
          :game-id="detectedGameId"
          :known-ranges="knownRanges"
          @new-range-detected="onNewRangeDetected"
        />
      </div>
    </div>
  </div>
</template>
