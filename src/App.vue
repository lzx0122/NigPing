<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import {
  Zap,
  Activity,
  ShieldCheck,
  Gamepad2,
  ChevronLeft,
  Server as ServerIcon,
  Play,
} from "lucide-vue-next";
import { GAMES, type Game, type Server } from "@/data/games";
import ServerGlobe from "@/components/ServerGlobe.vue";
import TitleBar from "@/components/TitleBar.vue";
import ServerDetection from "@/components/ServerDetection.vue";

// === 狀態管理 ===
type ViewState = "home" | "details";
const currentView = ref<ViewState>("home");
const selectedGame = ref<Game | null>(null);
const selectedServer = ref<Server | null>(null);

const status = ref("服務已就緒");
const isConnected = ref(false);
const isLoading = ref(false);
const currentPing = ref(0);

// === VPN 設定資料 ===
// 這些應該要放在安全的地方或是由後端提供，目前暫時 Hardcode
const PRIVATE_KEY = "8J3JPRxASoiS1pepU4NE3VoRUB8ILdpSBVKZrXt1uXs=";

// === 導航邏輯 ===
const goToDetails = (game: Game) => {
  selectedGame.value = game;
  // 預設選擇第一個伺服器
  if (game.servers.length > 0) {
    selectedServer.value = game.servers[0];
  }
  currentView.value = "details";
};

const goToHome = () => {
  currentView.value = "home";
  selectedGame.value = null;
};

// PUBG Asia Server Ranges (Korea, Japan, Singapore)
// 包含 AWS ap-northeast-2 (Seoul), ap-northeast-1 (Tokyo), ap-southeast-1 (Singapore)
// 以及 Azure Korea 和其他提供商
const PUBG_ASIA_RANGES = [
  // Korea (AWS)
  "13.124.0.0/16",
  "13.125.0.0/16",
  "13.209.0.0/16",
  "52.78.0.0/16",
  "52.79.0.0/16",
  // Korea (Azure)
  "20.194.0.0/16", // Azure Korea Central
  "20.196.0.0/16", // Azure Korea South
  "20.198.0.0/16",
  "20.200.0.0/16",
  "20.41.0.0/16",
  "52.231.0.0/16", // Azure Korea (主要範圍)
  "52.237.0.0/16", // Azure Korea (實際 PUBG 使用)
  // Singapore (AWS - Common for SEA region)
  "13.228.0.0/16",
  "18.136.0.0/16",
  "18.138.0.0/16",
  "52.74.0.0/16",
  "52.76.0.0/16",
  "54.169.0.0/16",
  "54.251.0.0/16",
  // Japan (AWS - Common fallback)
  "13.112.0.0/16",
  "52.192.0.0/16",
  "54.248.0.0/16",
  // Tencent Cloud / Other CDN (PUBG 實際使用)
  "85.236.96.0/22", // 包含 85.236.96.x - 85.236.99.x
  "103.28.54.0/24", // 東南亞遊戲伺服器
  // AWS Global Accelerator (PUBG 使用)
  "75.2.0.0/16", // AWS Global Accelerator range
  "99.77.0.0/16", // AWS Global Accelerator range
].join(", ");

const getWgConfig = (server: Server) => `
[Interface]
PrivateKey = ${PRIVATE_KEY}
Address = 10.8.0.2/24
DNS = 1.1.1.1
MTU = 1280

[Peer]
PublicKey = ${server.publicKey}
PresharedKey = JbLLJPvjfXhykHg8mDrNdonHhNTlAYZNh9v3u8bbNzI=
AllowedIPs = ${PUBG_ASIA_RANGES}
Endpoint = ${server.endpoint}
`;

const handleConnect = async () => {
  if (!selectedServer.value) return;

  isLoading.value = true;
  status.value = `🚀 正在連線至 ${selectedServer.value.name}...`;

  try {
    await invoke("connect_korea", {
      configContent: getWgConfig(selectedServer.value),
      ipv4Address: "10.8.0.2",
    });

    status.value = `✅ 已連線 (${selectedServer.value.name})`;
    isConnected.value = true;
    currentPing.value = Math.floor(Math.random() * 10) + 20; // 模擬 Ping 值 (台灣應該比較低)
  } catch (error) {
    console.error(error);
    status.value = "❌ 連線失敗: " + error;
  } finally {
    isLoading.value = false;
  }
};

const handleDisconnect = async () => {
  isLoading.value = true;
  status.value = "🛑 正在停止加速...";

  try {
    await invoke("disconnect_vpn");
    status.value = "服務已就緒";
    isConnected.value = false;
    currentPing.value = 0;
  } catch (error) {
    status.value = "❌ 停止失敗: " + error;
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <div
    class="h-screen w-full bg-zinc-950 text-white flex flex-col overflow-hidden font-sans"
  >
    <!-- Custom Title Bar -->
    <TitleBar />

    <!-- Main App Content -->
    <div class="flex-1 flex overflow-hidden">
      <aside class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
        <div
          class="p-6 flex items-center gap-2 cursor-pointer"
          @click="goToHome"
        >
          <Zap class="w-6 h-6 text-yellow-500" />
          <h1
            class="text-xl font-bold bg-gradient-to-r from-yellow-500 to-orange-500 bg-clip-text text-transparent"
          >
            NigPing
          </h1>
        </div>

        <nav class="flex-1 px-4 space-y-2">
          <Button
            variant="ghost"
            class="w-full justify-start gap-2"
            :class="
              currentView === 'home'
                ? 'bg-zinc-800 text-white'
                : 'text-zinc-400 hover:text-white hover:bg-zinc-800'
            "
            @click="goToHome"
          >
            <Gamepad2 class="w-4 h-4" />
            遊戲庫
          </Button>
          <Button
            variant="ghost"
            class="w-full justify-start gap-2 text-zinc-400 hover:text-white hover:bg-zinc-800"
          >
            <Activity class="w-4 h-4" />
            連線狀態
          </Button>
        </nav>

        <div class="p-4 border-t border-zinc-800 text-xs text-zinc-500">
          v0.1.0 Beta
        </div>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 flex flex-col relative">
        <!-- Top Bar -->
        <header
          class="h-16 border-b border-zinc-800 bg-zinc-900/50 backdrop-blur flex items-center justify-between px-8 z-10"
        >
          <div class="flex items-center gap-4">
            <!-- Back Button (Mobile style or deep nav) -->
            <Button
              v-if="currentView === 'details'"
              variant="ghost"
              size="icon"
              @click="goToHome"
              class="mr-2"
            >
              <ChevronLeft class="w-5 h-5" />
            </Button>

            <div
              class="flex items-center gap-2 px-3 py-1 rounded-full bg-zinc-800 border border-zinc-700"
            >
              <div
                class="w-2 h-2 rounded-full"
                :class="
                  isConnected
                    ? 'bg-green-500 shadow-[0_0_8px_#22c55e]'
                    : 'bg-red-500'
                "
              ></div>
              <span class="text-sm font-medium">{{ status }}</span>
            </div>
          </div>

          <div
            v-if="isConnected"
            class="flex items-center gap-4 text-green-400 text-sm font-mono"
          >
            <span>PING: {{ currentPing }}ms</span>
            <span>JITTER: 1ms</span>
          </div>
        </header>

        <!-- Content Area -->
        <div class="flex-1 p-8 overflow-auto">
          <!-- View: Home (Game List) -->
          <div v-if="currentView === 'home'">
            <h2 class="text-2xl font-bold mb-6">我的遊戲</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <Card
                v-for="game in GAMES"
                :key="game.id"
                class="bg-zinc-900 border-zinc-800 hover:border-zinc-500 transition-all duration-300 group relative overflow-hidden cursor-pointer"
                @click="goToDetails(game)"
              >
                <div
                  class="absolute inset-0 bg-cover bg-center opacity-40 group-hover:opacity-60 transition-opacity"
                  :style="{ backgroundImage: `url(${game.image})` }"
                ></div>
                <div
                  class="absolute inset-0 bg-gradient-to-t from-zinc-900 via-zinc-900/80 to-transparent"
                ></div>

                <CardHeader
                  class="relative z-10 pb-2 h-40 flex flex-col justify-end"
                >
                  <CardTitle class="text-2xl">{{ game.name }}</CardTitle>
                  <CardDescription
                    class="text-zinc-400 flex items-center gap-2"
                  >
                    <span class="w-2 h-2 rounded-full bg-green-500"></span>
                    可加速
                  </CardDescription>
                </CardHeader>
              </Card>

              <!-- Placeholder Card -->
              <Card
                class="bg-zinc-900/50 border-zinc-800 border-dashed flex items-center justify-center min-h-[160px] cursor-not-allowed opacity-50"
              >
                <div class="text-center text-zinc-500">
                  <p class="text-sm">更多遊戲敬請期待</p>
                </div>
              </Card>
            </div>
          </div>

          <!-- View: Game Details (Server Select) -->
          <div
            v-else-if="currentView === 'details' && selectedGame"
            class="max-w-4xl mx-auto"
          >
            <div class="flex items-start gap-6 h-full">
              <!-- Game Cover (smaller) -->
              <div
                class="w-48 aspect-[3/4] rounded-lg overflow-hidden shadow-2xl border border-zinc-800 relative flex-shrink-0"
              >
                <img
                  :src="selectedGame.image"
                  class="w-full h-full object-cover"
                />
                <div
                  class="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent flex items-end p-4"
                >
                  <span
                    class="text-xs bg-yellow-500 text-black px-2 py-1 rounded font-bold"
                    >Process Mode</span
                  >
                </div>
              </div>

              <!-- Controls -->
              <div class="flex-1 flex flex-col h-full">
                <div>
                  <h1 class="text-4xl font-bold mb-2">
                    {{ selectedGame.name }}
                  </h1>
                  <p class="text-zinc-400">請選擇加速節點並啟動服務</p>
                </div>

                <div class="grid grid-cols-2 gap-4 flex-1">
                  <!-- Left: Server Selector -->
                  <div
                    class="bg-zinc-900 p-4 rounded-lg border border-zinc-800 flex flex-col"
                  >
                    <label
                      class="text-sm text-zinc-500 font-medium flex items-center gap-2"
                    >
                      <ServerIcon class="w-4 h-4" /> 選擇節點
                    </label>

                    <div class="grid gap-2">
                      <button
                        v-for="server in selectedGame.servers"
                        :key="server.id"
                        @click="selectedServer = server"
                        class="flex items-center justify-between p-3 rounded border transition-all text-left"
                        :class="
                          selectedServer?.id === server.id
                            ? 'bg-zinc-800 border-yellow-500 text-white'
                            : 'bg-transparent border-zinc-700 text-zinc-400 hover:border-zinc-500'
                        "
                      >
                        <div class="flex items-center gap-3">
                          <span class="text-xl">{{ server.flag }}</span>
                          <span class="font-medium">{{ server.name }}</span>
                        </div>
                        <span class="text-xs text-zinc-500 font-mono">{{
                          server.region
                        }}</span>
                      </button>
                    </div>
                  </div>

                  <!-- Right: Server Globe Display -->
                  <div
                    v-if="selectedServer"
                    class="bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden relative min-h-[280px]"
                  >
                    <ServerGlobe
                      :markers="[
                        { location: selectedServer.location, size: 0.1 },
                      ]"
                    />
                  </div>
                </div>

                <div class="pt-4 border-t border-zinc-800 mt-4">
                  <Button
                    v-if="!isConnected"
                    @click="handleConnect"
                    :disabled="isLoading || !selectedServer"
                    size="lg"
                    class="w-full bg-gradient-to-r from-yellow-600 to-orange-600 hover:from-yellow-500 hover:to-orange-500 text-white font-bold h-14 text-lg shadow-lg shadow-orange-900/20"
                  >
                    <Play class="w-5 h-5 mr-2 fill-current" />
                    {{ isLoading ? "正在啟動..." : "立即加速" }}
                  </Button>
                  <Button
                    v-else
                    @click="handleDisconnect"
                    :disabled="isLoading"
                    variant="destructive"
                    size="lg"
                    class="w-full h-14 text-lg font-bold"
                  >
                    停止加速
                  </Button>

                  <p
                    class="text-center text-xs text-zinc-600 mt-3 flex items-center justify-center gap-2"
                  >
                    <ShieldCheck class="w-3 h-3" />
                    本連線採用 WireGuard 加密技術，保障您的隱私安全
                  </p>
                </div>

                <!-- Server Detection Component -->
                <div class="mt-6">
                  <ServerDetection
                    v-if="selectedGame"
                    :process-name="selectedGame.processName"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<style>
/* Custom Scrollbar */
::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-track {
  background: #18181b;
}
::-webkit-scrollbar-thumb {
  background: #3f3f46;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #52525b;
}
</style>
