<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { GAMES, type Game } from "@/data/games";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import ServerDetection from "../components/ServerDetection.vue";
import HopProbe from "../components/HopProbe.vue";

const selectedGameId = ref<string>(GAMES[0]?.id ?? "");

const selectedGame = computed<Game | undefined>(() =>
  GAMES.find((g) => g.id === selectedGameId.value),
);

const processName = computed(() => selectedGame.value?.processName ?? "");
const detectedGameId = computed(() => selectedGame.value?.id ?? "default");

const knownRanges = ref(new Set<string>());

watch(selectedGameId, () => {
  knownRanges.value = new Set();
});

function onNewRangeDetected(ip: string) {
  if (!ip || knownRanges.value.has(ip)) return;

  const nextRanges = new Set(knownRanges.value);
  nextRanges.add(ip);
  knownRanges.value = nextRanges;

  console.log(`[Dashboard] Added detected route: ${ip}`);
}
</script>

<template>
  <div class="h-full flex flex-col space-y-6 overflow-y-auto">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold text-white tracking-tight">Dashboard</h1>
    </div>

    <div class="space-y-4">
      <div
        class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between rounded-lg border border-zinc-800/50 bg-zinc-900/30 p-3 sm:px-4"
      >
        <div class="flex flex-col">
          <h2 class="text-sm font-bold text-white tracking-tight">
            Game Monitoring
          </h2>
          <div class="mt-1.5 flex items-center gap-2">
            <span class="text-xs font-medium text-zinc-500">Process target:</span>
            <span
              v-if="selectedGame"
              class="rounded border border-zinc-800 bg-zinc-950 px-1.5 py-0.5 font-mono text-[11px] text-zinc-400"
              :title="selectedGame.processName"
            >
              {{ selectedGame.processName }}
            </span>
          </div>
        </div>

        <div class="flex w-full flex-col gap-2 sm:w-auto sm:flex-row sm:items-center sm:gap-3">
          <Label
            for="dashboard-target-game"
            class="shrink-0 text-xs font-bold uppercase tracking-wider text-zinc-500"
          >
            Game
          </Label>
          <Select v-model="selectedGameId" :disabled="GAMES.length === 0">
            <SelectTrigger
              id="dashboard-target-game"
              class="h-9 w-full border-zinc-800 bg-zinc-950 text-white shadow-sm ring-offset-zinc-950 hover:border-zinc-700 focus:ring-zinc-600 data-[placeholder]:text-zinc-500 sm:w-56"
            >
              <SelectValue placeholder="Choose a game" />
            </SelectTrigger>
            <SelectContent
              class="border-zinc-800 bg-zinc-950 text-zinc-100"
            >
              <SelectItem
                v-for="g in GAMES"
                :key="g.id"
                :value="g.id"
                class="focus:bg-zinc-800 focus:text-white data-[highlighted]:bg-zinc-800 data-[highlighted]:text-white"
              >
                {{ g.name }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>
      </div>

      <ServerDetection
        :key="selectedGameId"
        :process-name="processName"
        :game-id="detectedGameId"
        :known-ranges="knownRanges"
        @new-range-detected="onNewRangeDetected"
      />

      <HopProbe />
    </div>
  </div>
</template>
