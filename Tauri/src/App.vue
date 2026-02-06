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
  Gamepad2,
  ChevronLeft,
  Server as ServerIcon,
  Play,
  StopCircle,
  List,
  X,
  LogOut,
} from "lucide-vue-next";
import { GAMES, type Game, type Server } from "@/data/games";
import ServerGlobe from "@/components/ServerGlobe.vue";
import TitleBar from "@/components/TitleBar.vue";
import ServerDetection from "@/components/ServerDetection.vue";
import Login from "@/components/Login.vue";
import { useAuth } from "@/composables/useAuth";

const { isAuthenticated, logout } = useAuth();

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

  // Fetch IP ranges for this game
  fetchGameRanges(game.id);

  currentView.value = "details";
};

const goToHome = () => {
  currentView.value = "home";
  selectedGame.value = null;
};

const gameIpRanges = ref<Set<string>>(new Set());
const showRanges = ref(false);

// Fetch IP ranges for selected game
const fetchGameRanges = async (gameId: string) => {
  try {
    const ranges = (await fetch(
      `http://localhost:3000/api/games/${gameId}/ranges`,
    ).then((res) => res.json())) as string[];

    // Clear and add new ranges
    gameIpRanges.value.clear();
    ranges.forEach((range) => gameIpRanges.value.add(range));

    console.log(`Fetched ${ranges.length} IP ranges for ${gameId}`);
  } catch (error) {
    console.error(`Failed to fetch IP ranges for ${gameId}:`, error);
  }
};

const getWgConfig = (server: Server) => {
  const allowedIps = Array.from(gameIpRanges.value).join(", ");

  return `
[Interface]
PrivateKey = ${PRIVATE_KEY}
Address = 10.8.0.2/24
DNS = 1.1.1.1
MTU = 1280

[Peer]
PublicKey = ${server.publicKey}
PresharedKey = JbLLJPvjfXhykHg8mDrNdonHhNTlAYZNh9v3u8bbNzI=
AllowedIPs = ${allowedIps}
Endpoint = ${server.endpoint}
`;
};

const handleConnect = async () => {
  if (!selectedServer.value) return;

  isLoading.value = true;
  status.value = `正在連線至 ${selectedServer.value.name}...`;

  try {
    await invoke("connect_korea", {
      configContent: getWgConfig(selectedServer.value),
      ipv4Address: "10.8.0.2",
    });

    status.value = `已連線 - ${selectedServer.value.name}`;
    isConnected.value = true;
    currentPing.value = Math.floor(Math.random() * 10) + 20; // 模擬 Ping 值 (台灣應該比較低)
  } catch (error) {
    console.error(error);
    status.value = "連線失敗: " + error;
  } finally {
    isLoading.value = false;
  }
};

const handleDisconnect = async () => {
  isLoading.value = true;
  status.value = "正在停止加速...";

  try {
    await invoke("disconnect_vpn");
    status.value = "服務已就緒";
    isConnected.value = false;
    currentPing.value = 0;
  } catch (error) {
    status.value = "停止失敗: " + error;
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <Login v-if="!isAuthenticated" />
  <div
    v-else
    class="h-screen w-full bg-black text-zinc-100 flex flex-col overflow-hidden font-sans selection:bg-white selection:text-black"
  >
    <!-- Custom Title Bar -->
    <TitleBar />

    <!-- Main App Content -->
    <div class="flex-1 flex overflow-hidden">
      <aside class="w-64 bg-black border-r border-zinc-900 flex flex-col">
        <div
          class="p-6 flex items-center gap-3 cursor-pointer group"
          @click="goToHome"
        >
          <div
            class="w-8 h-8 flex items-center justify-center bg-white text-black rounded-md shadow-lg shadow-zinc-800/20 group-hover:scale-105 transition-transform duration-300"
          >
            <Zap class="w-5 h-5 fill-current" />
          </div>
          <div>
            <h1
              class="text-lg font-bold text-white tracking-tight leading-none"
            >
              NigPing
            </h1>
            <span
              class="text-[10px] text-zinc-500 font-mono tracking-widest uppercase"
              >Accelerator</span
            >
          </div>
        </div>

        <nav class="flex-1 px-3 space-y-1">
          <Button
            variant="ghost"
            class="w-full justify-start gap-3 h-10 font-medium"
            :class="
              currentView === 'home'
                ? 'bg-zinc-900 text-white'
                : 'text-zinc-500 hover:text-zinc-200 hover:bg-zinc-900/50'
            "
            @click="goToHome"
          >
            <Gamepad2 class="w-4 h-4" />
            遊戲庫
          </Button>
          <Button
            variant="ghost"
            class="w-full justify-start gap-3 h-10 font-medium text-zinc-500 hover:text-zinc-200 hover:bg-zinc-900/50"
          >
            <Activity class="w-4 h-4" />
            連線狀態
          </Button>
        </nav>

        <div class="p-6 border-t border-zinc-900 space-y-3">
          <Button
            variant="ghost"
            class="w-full justify-start gap-3 h-10 font-medium text-zinc-500 hover:text-red-400 hover:bg-zinc-900/50"
            @click="logout"
          >
            <LogOut class="w-4 h-4" />
            登出
          </Button>
          <div
            class="flex items-center justify-between text-xs text-zinc-600 font-mono"
          >
            <span>v0.1.0 Beta</span>
            <div class="w-2 h-2 rounded-full bg-zinc-800 animate-pulse"></div>
          </div>
        </div>
      </aside>

      <!-- Main Content -->
      <main class="flex-1 flex flex-col relative bg-zinc-950">
        <!-- Top Bar -->
        <header
          class="h-16 border-b border-zinc-900 bg-black/80 backdrop-blur flex items-center justify-between px-8 z-10 sticky top-0"
        >
          <div class="flex items-center gap-4">
            <!-- Back Button -->
            <Button
              v-if="currentView === 'details'"
              variant="outline"
              size="icon"
              @click="goToHome"
              class="w-8 h-8 rounded-full border-zinc-800 bg-black hover:bg-zinc-900 hover:border-zinc-700 transition-all mr-2"
            >
              <ChevronLeft class="w-4 h-4" />
            </Button>

            <div
              class="flex items-center gap-3 px-4 py-1.5 rounded-full bg-zinc-900 border border-zinc-800"
            >
              <div
                class="relative flex items-center justify-center w-2.5 h-2.5"
              >
                <div
                  class="absolute w-full h-full rounded-full animate-ping opacity-75"
                  :class="isConnected ? 'bg-white' : 'bg-transparent'"
                ></div>
                <div
                  class="relative w-2 h-2 rounded-full transition-colors duration-500"
                  :class="
                    isConnected
                      ? 'bg-white shadow-[0_0_8px_white]'
                      : 'bg-zinc-600'
                  "
                ></div>
              </div>
              <span class="text-xs font-medium text-zinc-300 tracking-wide">{{
                status
              }}</span>
            </div>
          </div>

          <div v-if="isConnected" class="flex items-center gap-6">
            <div class="flex flex-col items-end">
              <span
                class="text-[10px] text-zinc-500 font-bold uppercase tracking-wider"
                >Ping</span
              >
              <span class="text-lg font-mono font-bold leading-none"
                >{{ currentPing
                }}<span class="text-sm text-zinc-600 ml-1">ms</span></span
              >
            </div>
            <div class="h-8 w-px bg-zinc-800"></div>
            <div class="flex flex-col items-end">
              <span
                class="text-[10px] text-zinc-500 font-bold uppercase tracking-wider"
                >Jitter</span
              >
              <span class="text-lg font-mono font-bold leading-none"
                >1<span class="text-sm text-zinc-600 ml-1">ms</span></span
              >
            </div>
          </div>
        </header>

        <!-- Content Area -->
        <div class="flex-1 p-8 overflow-auto">
          <!-- View: Home (Game List) -->
          <div v-if="currentView === 'home'" class="max-w-6xl mx-auto">
            <div
              class="flex items-end justify-between mb-8 border-b border-zinc-900 pb-4"
            >
              <h2 class="text-3xl font-bold tracking-tight text-white">
                遊戲庫
              </h2>
              <span class="text-sm text-zinc-500"
                >{{ GAMES.length }} GAMES AVAILABLE</span
              >
            </div>

            <div
              class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
            >
              <Card
                v-for="game in GAMES"
                :key="game.id"
                class="bg-zinc-900 border-zinc-800 hover:border-zinc-600 hover:bg-zinc-800 transition-all duration-300 group relative overflow-hidden cursor-pointer h-[280px]"
                @click="goToDetails(game)"
              >
                <!-- Image with monochrome filter removed -->
                <div
                  class="absolute inset-0 bg-cover bg-center opacity-60 group-hover:opacity-80 transition-all duration-500"
                  :style="{ backgroundImage: `url(${game.image})` }"
                ></div>
                <div
                  class="absolute inset-0 bg-gradient-to-t from-black via-black/80 to-transparent"
                ></div>

                <div class="absolute top-4 right-4 z-10">
                  <div
                    class="w-8 h-8 rounded-full bg-white text-black flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all duration-300 transform translate-y-2 group-hover:translate-y-0 shadow-lg"
                  >
                    <Play class="w-4 h-4 fill-current ml-0.5" />
                  </div>
                </div>

                <CardHeader
                  class="relative z-10 h-full flex flex-col justify-end p-6"
                >
                  <CardTitle class="text-xl font-bold text-white mb-1">{{
                    game.name
                  }}</CardTitle>
                  <CardDescription
                    class="text-zinc-500 text-xs flex items-center gap-2 uppercase tracking-wide font-medium"
                  >
                    <span class="w-1.5 h-1.5 rounded-full bg-zinc-500"></span>
                    Ready to accelerate
                  </CardDescription>
                </CardHeader>
              </Card>

              <!-- Placeholder Card -->
              <div
                class="border border-dashed border-zinc-800 rounded-xl flex flex-col items-center justify-center h-[280px] text-zinc-600 hover:border-zinc-700 hover:text-zinc-500 transition-colors cursor-default"
              >
                <Gamepad2 class="w-8 h-8 mb-3 opacity-20" />
                <p class="text-xs font-medium uppercase tracking-widest">
                  More Coming Soon
                </p>
              </div>
            </div>
          </div>

          <!-- View: Game Details (Server Select) -->
          <div
            v-else-if="currentView === 'details' && selectedGame"
            class="max-w-5xl mx-auto flex flex-col"
          >
            <div class="flex items-start gap-8">
              <!-- Game Cover (smaller) -->
              <div
                class="w-64 aspect-[3/4] rounded-lg overflow-hidden bg-zinc-900 border border-zinc-800 relative flex-shrink-0 shadow-2xl"
              >
                <img
                  :src="selectedGame.image"
                  class="w-full h-full object-cover opacity-80"
                />
                <div
                  class="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent"
                ></div>
                <div class="absolute bottom-5 left-5 right-5">
                  <h1
                    class="text-2xl font-bold leading-tight mb-2 text-white drop-shadow-md"
                  >
                    {{ selectedGame.name }}
                  </h1>
                  <div
                    class="inline-flex items-center px-2 py-1 rounded bg-white text-black text-[10px] font-bold uppercase tracking-wider"
                  >
                    Process Mode
                  </div>
                </div>
              </div>

              <!-- Controls -->
              <div class="flex-1 flex flex-col gap-6">
                <div
                  class="grid grid-cols-1 lg:grid-cols-2 gap-6 flex-1 min-h-0"
                >
                  <!-- Left: Server Selector -->
                  <div class="flex flex-col gap-3 min-h-0">
                    <div class="flex items-center justify-between">
                      <label
                        class="text-xs font-bold text-zinc-500 uppercase tracking-widest flex items-center gap-2"
                      >
                        <ServerIcon class="w-3.5 h-3.5" /> 節點選擇
                      </label>
                      <Button
                        variant="ghost"
                        size="sm"
                        class="h-6 px-2 text-[10px] text-zinc-500 hover:text-zinc-300"
                        @click="showRanges = true"
                      >
                        <List class="w-3 h-3 mr-1" />
                        顯示 IP 範圍
                      </Button>
                    </div>

                    <div
                      class="flex flex-col gap-2 overflow-y-auto pr-2 custom-scrollbar flex-1 pb-2"
                    >
                      <button
                        v-for="server in selectedGame.servers"
                        :key="server.id"
                        @click="selectedServer = server"
                        class="group flex items-center justify-between p-4 rounded-lg border transition-all text-left relative overflow-hidden"
                        :class="
                          selectedServer?.id === server.id
                            ? 'bg-white text-black border-white shadow-lg shadow-white/10'
                            : 'bg-zinc-900/50 border-zinc-800 text-zinc-400 hover:border-zinc-600 hover:bg-zinc-900'
                        "
                      >
                        <div
                          v-if="selectedServer?.id === server.id"
                          class="absolute inset-0 bg-gradient-to-r from-white/10 via-transparent to-transparent pointer-events-none"
                        ></div>
                        <div class="flex items-center gap-4 relative z-10">
                          <span class="text-2xl transition-all">{{
                            server.flag
                          }}</span>
                          <div class="flex flex-col">
                            <span class="font-bold text-sm tracking-tight">{{
                              server.name
                            }}</span>
                            <span class="text-xs font-mono opacity-60">{{
                              server.region
                            }}</span>
                          </div>
                        </div>
                        <div
                          v-if="selectedServer?.id === server.id"
                          class="w-2 h-2 rounded-full bg-black relative z-10"
                        ></div>
                      </button>
                    </div>
                  </div>

                  <!-- Right: Visualizer & Actions -->
                  <div class="flex flex-col gap-6">
                    <!-- Server Globe Display -->
                    <div
                      v-if="selectedServer"
                      class="bg-zinc-900/30 rounded-lg border border-zinc-800 overflow-hidden relative h-[250px] shadow-inner"
                    >
                      <!-- Ideally the globe should also be monochrome, likely configured via props in real implementation -->
                      <ServerGlobe
                        :markers="[
                          { location: selectedServer.location, size: 0.1 },
                        ]"
                      />
                      <div
                        class="absolute inset-0 pointer-events-none border border-white/5 rounded-lg"
                      ></div>
                    </div>

                    <!-- Action Button -->
                    <div class="mt-auto">
                      <Button
                        v-if="!isConnected"
                        @click="handleConnect"
                        :disabled="isLoading || !selectedServer"
                        size="lg"
                        class="w-full bg-white text-black hover:bg-zinc-200 border border-transparent font-bold h-14 text-base tracking-wide shadow-lg shadow-zinc-900/50 transition-all active:scale-[0.98]"
                      >
                        <Play
                          v-if="!isLoading"
                          class="w-4 h-4 mr-2 fill-current"
                        />
                        <div
                          v-else
                          class="w-4 h-4 mr-2 border-2 border-black/30 border-t-black rounded-full animate-spin"
                        ></div>
                        {{ isLoading ? "正在啟動..." : "立即加速" }}
                      </Button>

                      <Button
                        v-else
                        @click="handleDisconnect"
                        :disabled="isLoading"
                        variant="destructive"
                        size="lg"
                        class="w-full h-14 text-base font-bold bg-zinc-900 border border-zinc-800 hover:bg-red-950 hover:border-red-900 hover:text-red-500 text-zinc-300 transition-all active:scale-[0.98]"
                      >
                        <StopCircle v-if="!isLoading" class="w-4 h-4 mr-2" />
                        <div
                          v-else
                          class="w-4 h-4 mr-2 border-2 border-white/30 border-t-white rounded-full animate-spin"
                        ></div>
                        停止加速
                      </Button>
                    </div>
                  </div>
                </div>

                <!-- Server Detection Component -->
                <div class="pt-6 border-t border-zinc-900">
                  <div class="w-full">
                    <ServerDetection
                      v-if="selectedGame"
                      :process-name="selectedGame.processName"
                      :game-id="selectedGame.id"
                      :known-ranges="gameIpRanges"
                      @new-range-detected="fetchGameRanges(selectedGame.id)"
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>

      <!-- IP Ranges Modal -->
      <div
        v-if="showRanges"
        class="absolute inset-0 bg-black/80 backdrop-blur-sm z-50 flex items-center justify-center p-8"
        @click.self="showRanges = false"
      >
        <div
          class="bg-zinc-900 border border-zinc-800 rounded-xl w-full max-w-lg shadow-2xl flex flex-col max-h-[80vh]"
        >
          <div
            class="flex items-center justify-between p-6 border-b border-zinc-800"
          >
            <h3 class="text-lg font-bold text-white">Known IP Ranges</h3>
            <button
              @click="showRanges = false"
              class="text-zinc-500 hover:text-white"
            >
              <X class="w-5 h-5" />
            </button>
          </div>
          <div class="p-6 overflow-y-auto custom-scrollbar">
            <div class="grid grid-cols-2 gap-3">
              <div
                v-for="range in Array.from(gameIpRanges).sort()"
                :key="range"
                class="bg-black/50 border border-zinc-800/50 rounded px-3 py-2 text-zinc-300 font-mono text-sm flex items-center gap-2"
              >
                <div class="w-1.5 h-1.5 rounded-full bg-emerald-500"></div>
                {{ range }}
              </div>
              <div
                v-if="gameIpRanges.size === 0"
                class="col-span-2 text-center text-zinc-500 py-8 italic"
              >
                No ranges fetched from backend.
              </div>
            </div>
          </div>
          <div
            class="p-4 border-t border-zinc-800 bg-zinc-950/50 rounded-b-xl text-xs text-zinc-500 text-center"
          >
            Total: {{ gameIpRanges.size }} ranges
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
/* Custom Scrollbar for the entire app */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: #3f3f46;
}

/* Specific utilities */
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
</style>
