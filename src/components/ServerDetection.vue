<template>
  <div
    class="flex flex-col w-full rounded-lg border border-zinc-800 bg-zinc-950/50 backdrop-blur-sm overflow-hidden shadow-sm"
  >
    <!-- Header with Monitoring Toggle (Compact) -->
    <div
      class="flex items-center justify-between p-3 border-b border-zinc-800 bg-zinc-900/20"
    >
      <div class="flex items-center gap-3">
        <div
          class="w-8 h-8 flex items-center justify-center rounded bg-zinc-900 border border-zinc-800 text-white shadow-sm"
        >
          <Activity
            :class="{
              'animate-pulse text-white': isMonitoring,
              'text-zinc-500': !isMonitoring,
            }"
            class="w-4 h-4"
          />
        </div>
        <div class="flex flex-col">
          <h3 class="text-sm font-bold text-white tracking-tight leading-none">
            伺服器偵測
          </h3>
          <p class="text-[10px] text-zinc-500 font-medium leading-tight mt-0.5">
            自動識別遊戲連線
          </p>
        </div>
      </div>

      <button
        @click="toggleMonitoring"
        :disabled="isLoading"
        class="flex items-center gap-1.5 px-3 py-1.5 rounded text-xs font-medium transition-all duration-200 border"
        :class="[
          isMonitoring
            ? 'bg-white text-black border-white hover:bg-zinc-200 hover:border-zinc-200 shadow-sm'
            : 'bg-zinc-900 text-zinc-400 border-zinc-800 hover:bg-zinc-800 hover:text-white hover:border-zinc-700',
        ]"
      >
        <Zap v-if="isMonitoring" class="w-3 h-3 fill-current" />
        <Power v-else class="w-3 h-3" />
        <span>{{ isMonitoring ? "監控中" : "啟動" }}</span>
      </button>
    </div>

    <!-- Status Banner (Compact) -->
    <div
      v-if="statusMessage"
      class="flex items-center gap-2 px-3 py-1.5 border-b border-zinc-800/50 text-xs font-medium"
      :class="{
        'bg-zinc-900/50 text-zinc-300': statusType === 'info',
        'bg-zinc-900/50 text-white': statusType === 'success',
        'bg-red-950/20 text-red-400': statusType === 'error',
      }"
    >
      <div
        class="w-1.5 h-1.5 rounded-full shrink-0"
        :class="
          statusType === 'success'
            ? 'bg-green-500'
            : statusType === 'error'
              ? 'bg-red-500'
              : 'bg-zinc-500'
        "
      ></div>
      <span class="truncate opacity-90">{{ statusMessage }}</span>
    </div>

    <!-- Detected Servers List (Compact) -->
    <div v-if="detectedServers.length > 0" class="p-3 flex flex-col gap-2">
      <!-- Primary Server (Compact Layout) -->
      <div v-if="primaryServer" class="flex flex-col gap-2">
        <div
          class="flex items-center justify-between text-[10px] font-bold uppercase tracking-wider text-zinc-500 px-1"
        >
          <span class="flex items-center gap-1.5"
            ><Trophy class="w-3 h-3" /> 主要伺服器</span
          >
          <span class="font-mono">{{ primaryServer.protocol }}</span>
        </div>

        <div
          class="group relative flex flex-col p-3 rounded bg-zinc-900/40 border border-zinc-800 hover:border-zinc-600 hover:bg-zinc-900/60 transition-all gap-3"
        >
          <!-- Top Row: IP & Info -->
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3 overflow-hidden">
              <div
                class="w-8 h-8 rounded bg-zinc-800 flex items-center justify-center border border-zinc-700 shrink-0"
              >
                <Globe class="w-4 h-4 text-zinc-300" />
              </div>
              <div class="flex flex-col overflow-hidden">
                <div class="flex items-baseline gap-1.5">
                  <span
                    class="text-sm font-mono font-bold text-white truncate"
                    >{{ primaryServer.ip }}</span
                  >
                  <span class="text-[10px] font-mono text-zinc-500"
                    >:{{ primaryServer.port }}</span
                  >
                </div>
                <div class="flex items-center gap-2 text-[10px] text-zinc-400">
                  <span
                    v-if="primaryServer.country"
                    class="truncate flex items-center gap-1"
                    ><MapPin class="w-3 h-3" />{{ primaryServer.country }}</span
                  >
                </div>
              </div>
            </div>

            <button
              @click="addToRoutes(primaryServer.ip)"
              :disabled="addingRoute === primaryServer.ip"
              class="flex items-center justify-center w-7 h-7 rounded bg-white text-black hover:bg-zinc-200 transition-colors disabled:opacity-50"
              title="加入路由"
            >
              <Plus v-if="addingRoute !== primaryServer.ip" class="w-4 h-4" />
              <Loader2 v-else class="w-4 h-4 animate-spin" />
            </button>
          </div>

          <!-- Middle: Sparkline History -->
          <div
            class="h-10 w-full bg-zinc-950/50 rounded border border-zinc-800/50 relative overflow-hidden flex items-end"
          >
            <svg class="w-full h-full" preserveAspectRatio="none">
              <!-- Send Rate Line -->
              <path
                :d="sendPath"
                fill="none"
                stroke="#a1a1aa"
                stroke-width="1.5"
                vector-effect="non-scaling-stroke"
                class="opacity-50"
              />
              <!-- Recv Rate Line -->
              <path
                :d="recvPath"
                fill="none"
                stroke="#fff"
                stroke-width="1.5"
                vector-effect="non-scaling-stroke"
              />
            </svg>
            <div
              class="absolute top-1 right-1 text-[9px] font-mono text-zinc-600 bg-zinc-950/80 px-1 rounded"
            >
              traffic history
            </div>
          </div>

          <!-- Bottom Row: Stats Grid -->
          <div class="grid grid-cols-4 gap-2 pt-1 border-t border-zinc-800/50">
            <div class="flex flex-col">
              <span class="text-[9px] text-zinc-500 uppercase">Upload</span>
              <span class="text-xs font-mono text-zinc-300"
                >{{ formatBytes(primaryServer.send_rate) }}/s</span
              >
            </div>
            <div class="flex flex-col">
              <span class="text-[9px] text-zinc-500 uppercase">Download</span>
              <span class="text-xs font-mono text-white"
                >{{ formatBytes(primaryServer.recv_rate) }}/s</span
              >
            </div>
            <div class="flex flex-col">
              <span class="text-[9px] text-zinc-500 uppercase">Ping</span>
              <span class="text-xs font-mono text-zinc-400">--</span>
            </div>
            <div class="flex flex-col">
              <span class="text-[9px] text-zinc-500 uppercase">Loss</span>
              <span class="text-xs font-mono text-zinc-400">0%</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Others (Compact) -->
      <div v-if="otherServers.length > 0" class="flex flex-col gap-1 mt-1">
        <button
          @click="showMore = !showMore"
          class="flex items-center gap-2 text-[10px] font-medium text-zinc-600 hover:text-zinc-400 transition-colors py-1 px-1"
        >
          <ChevronDown
            :class="{ 'rotate-180': showMore }"
            class="w-3 h-3 transition-transform duration-200"
          />
          <span>{{
            showMore ? "收起" : `其他 ${otherServers.length} 個連線`
          }}</span>
        </button>

        <div v-if="showMore" class="flex flex-col gap-1.5 pl-1">
          <div
            v-for="server in otherServers"
            :key="server.ip + server.port"
            class="flex items-center justify-between p-2 rounded border border-zinc-800/30 bg-zinc-900/10 hover:bg-zinc-900/30 hover:border-zinc-700/50 transition-all text-xs"
          >
            <div class="flex items-center gap-2 overflow-hidden">
              <div
                class="w-1 h-1 rounded-full bg-zinc-500 shrink-0"
                :class="{
                  'bg-white shadow-[0_0_4px_white]': server.is_game_server,
                }"
              ></div>
              <div class="flex items-baseline gap-1 truncate">
                <span class="font-mono text-zinc-300">{{ server.ip }}</span>
              </div>
            </div>
            <div class="font-mono text-zinc-500">
              {{ formatBytes(server.send_rate + server.recv_rate) }}/s
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State (Compact) -->
    <div
      v-else-if="isMonitoring"
      class="flex flex-col items-center justify-center py-8 px-4 text-center"
    >
      <Loader2 class="w-5 h-5 text-zinc-600 animate-spin mb-2" />
      <p class="text-zinc-400 text-xs font-medium">Scanning...</p>
    </div>

    <div
      v-else
      class="flex flex-col items-center justify-center py-8 px-4 text-center"
    >
      <div
        class="w-8 h-8 rounded-full bg-zinc-900/50 flex items-center justify-center mb-2 border border-zinc-800/50"
      >
        <Target class="w-4 h-4 text-zinc-600" />
      </div>
      <p class="text-zinc-500 text-xs text-center leading-tight">
        Ready to Monitor
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  Activity,
  Zap,
  Power,
  Info,
  CheckCircle2,
  AlertTriangle,
  Globe,
  MapPin,
  ArrowUp,
  ArrowDown,
  Plus,
  Loader2,
  Target,
  Trophy,
  ChevronDown,
} from "lucide-vue-next";

interface DetectedServer {
  ip: string;
  port: number;
  protocol: string;
  send_rate: number;
  recv_rate: number;
  country: string | null;
  detected_at: string;
  is_game_server: boolean;
}

// Props
interface Props {
  processName: string; // e.g., "TslGame.exe"
}

defineProps<Props>();

// State
const isMonitoring = ref(false);
const isLoading = ref(false);
const detectedServers = ref<DetectedServer[]>([]);
const statusMessage = ref("");
const statusType = ref<"info" | "success" | "error">("info");
const addingRoute = ref<string | null>(null);
const showMore = ref(false);

// History State
const MAX_HISTORY = 30; // Keep last 30 data points
interface DataPoint {
  time: number;
  send: number;
  recv: number;
}
const history = ref<DataPoint[]>([]);

let pollInterval: number | null = null;

// Computed for Primary vs Others
const sortedServers = computed(() => {
  // 1. Filter only Game Servers (UDP)
  const udpServers = detectedServers.value.filter((s) => s.is_game_server);

  // 2. Sort by total traffic rate descending
  return udpServers.sort((a, b) => {
    const rateA = a.send_rate + a.recv_rate;
    const rateB = b.send_rate + b.recv_rate;
    return rateB - rateA;
  });
});

const primaryServer = computed(() => {
  if (sortedServers.value.length === 0) return null;
  // The top one is the primary
  return sortedServers.value[0];
});

const otherServers = computed(() => {
  if (sortedServers.value.length <= 1) return [];
  return sortedServers.value.slice(1);
});

// Watch primary server to push history
watch(primaryServer, (newVal) => {
  if (newVal) {
    history.value.push({
      time: Date.now(),
      send: newVal.send_rate,
      recv: newVal.recv_rate,
    });
    // Limit history size
    if (history.value.length > MAX_HISTORY) {
      history.value.shift();
    }
  }
});

// Sparkline Paths
const sendPath = computed(() => createSparklinePath("send"));
const recvPath = computed(() => createSparklinePath("recv"));

function createSparklinePath(key: "send" | "recv") {
  if (history.value.length < 2) return "";

  const width = 100; // viewbox 100x100
  const height = 100;

  // Find max value to scale Y axis (min 1KB to avoid flatline at 0)
  const maxVal = Math.max(
    ...history.value.map((d) => Math.max(d.send, d.recv)),
    1024,
  );

  const points = history.value.map((d, i) => {
    const x = (i / (history.value.length - 1)) * width;
    const y = height - (d[key] / maxVal) * height; // Invert Y
    return `${x.toFixed(1)},${y.toFixed(1)}`;
  });

  return `M ${points.join(" L ")}`;
}

// Methods
async function toggleMonitoring() {
  if (isMonitoring.value) {
    await stopMonitoring();
  } else {
    await startMonitoring();
  }
}

async function startMonitoring() {
  isLoading.value = true;
  statusMessage.value = "啟動中...";
  statusType.value = "info";
  history.value = []; // Reset history

  try {
    await invoke("start_monitoring", {
      processName: "TslGame.exe", // TODO: Use prop
    });

    isMonitoring.value = true;
    statusMessage.value = "掃描中...";
    statusType.value = "info";

    // Start polling for detected servers
    pollInterval = window.setInterval(fetchServers, 2000);

    // Also fetch immediately
    await fetchServers();
  } catch (error) {
    statusMessage.value = `失敗: ${error}`;
    statusType.value = "error";
    console.error("Failed to start monitoring:", error);
  } finally {
    isLoading.value = false;
  }
}

async function stopMonitoring() {
  isLoading.value = true;
  try {
    await invoke("stop_monitoring");
    isMonitoring.value = false;
    statusMessage.value = "已停止";
    statusType.value = "info";

    if (pollInterval !== null) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
    detectedServers.value = [];
    history.value = [];
  } catch (error) {
    statusMessage.value = `停止失敗: ${error}`;
    statusType.value = "error";
    console.error("Failed to stop monitoring:", error);
  } finally {
    isLoading.value = false;
  }
}

async function fetchServers() {
  try {
    const servers = await invoke<DetectedServer[]>("get_detected_servers");
    detectedServers.value = servers;

    if (servers.length > 0 && isMonitoring.value) {
      const gameServers = servers.filter((s) => s.is_game_server);
      if (gameServers.length > 0) {
        statusMessage.value = "已連線";
        statusType.value = "success";
      } else {
        statusMessage.value = "掃描中...";
        statusType.value = "info";
      }
    } else if (isMonitoring.value) {
      statusMessage.value = "掃描中...";
      statusType.value = "info";
    }
  } catch (error) {
    console.error("Failed to fetch servers:", error);
  }
}

async function addToRoutes(ip: string) {
  addingRoute.value = ip;
  try {
    const result = await invoke<string>("add_detected_ip_to_routes", { ip });
    statusMessage.value = "已加入路由"; // Shortened
    statusType.value = "success";
  } catch (error) {
    statusMessage.value = `失敗: ${error}`;
    statusType.value = "error";
  } finally {
    addingRoute.value = null;
  }
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
}

function formatTime(isoString: string): string {
  const date = new Date(isoString);
  const now = new Date();
  const diff = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diff < 60) return `${diff}s`;
  if (diff < 3600) return `${Math.floor(diff / 60)}m`;
  return date.toLocaleTimeString("zh-TW", {
    hour: "2-digit",
    minute: "2-digit",
  });
}

// Cleanup
onUnmounted(() => {
  if (pollInterval !== null) {
    clearInterval(pollInterval);
  }
});
</script>

<style scoped>
@import url("https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600&family=Noto+Sans+TC:wght@400;500;700&display=swap");
</style>
