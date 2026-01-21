<template>
  <div class="server-detection-panel">
    <!-- Header with Monitoring Toggle -->
    <div class="panel-header">
      <div class="header-content">
        <div class="header-icon">
          <Activity :class="{ pulse: isMonitoring }" />
        </div>
        <div class="header-text">
          <h3 class="panel-title">伺服器偵測</h3>
          <p class="panel-subtitle">自動識別遊戲連線</p>
        </div>
      </div>

      <button
        @click="toggleMonitoring"
        :disabled="isLoading"
        class="toggle-btn"
        :class="{ active: isMonitoring, loading: isLoading }"
      >
        <Zap v-if="isMonitoring" class="btn-icon" />
        <Power v-else class="btn-icon" />
        <span>{{ isMonitoring ? "監控中" : "啟動偵測" }}</span>
      </button>
    </div>

    <!-- Status Banner -->
    <div v-if="statusMessage" class="status-banner" :class="statusType">
      <Info v-if="statusType === 'info'" class="status-icon" />
      <CheckCircle2 v-else-if="statusType === 'success'" class="status-icon" />
      <AlertTriangle v-else class="status-icon" />
      <span>{{ statusMessage }}</span>
    </div>

    <!-- Detected Servers List -->
    <div v-if="detectedServers.length > 0" class="servers-container">
      <!-- Primary Server (Highest Usage Game Server) -->
      <div v-if="primaryServer" class="primary-server-section">
        <div class="section-label">
          <Trophy class="label-icon" /> 主要遊戲伺服器 (高流量)
        </div>

        <div class="server-card primary is-game">
          <div class="server-type-badge">
            <span class="badge game">
              {{ primaryServer.protocol }} 遊戲主機
            </span>
          </div>

          <div class="server-main-info">
            <div class="server-ip-section">
              <Globe class="ip-icon" />
              <div class="ip-details">
                <span class="ip-address big">{{ primaryServer.ip }}</span>
                <span class="ip-port">:{{ primaryServer.port }}</span>
              </div>
            </div>

            <div v-if="primaryServer.country" class="server-location">
              <MapPin class="location-icon" />
              <span>{{ primaryServer.country }}</span>
            </div>
          </div>

          <div class="server-stats large">
            <div class="stat">
              <ArrowUp class="stat-icon upload" />
              <span>{{ formatBytes(primaryServer.send_rate) }}/s</span>
            </div>
            <div class="stat">
              <ArrowDown class="stat-icon download" />
              <span>{{ formatBytes(primaryServer.recv_rate) }}/s</span>
            </div>
          </div>

          <div class="server-actions">
            <button
              @click="addToRoutes(primaryServer.ip)"
              :disabled="addingRoute === primaryServer.ip"
              class="add-route-btn primary-btn"
            >
              <Plus
                v-if="addingRoute !== primaryServer.ip"
                class="btn-icon-sm"
              />
              <Loader2 v-else class="btn-icon-sm spin" />
              加入路由
            </button>
            <span class="detected-time">{{
              formatTime(primaryServer.detected_at)
            }}</span>
          </div>
        </div>
      </div>

      <!-- Divider / Header for others -->
      <div class="servers-subheader">
        <div class="divider"></div>
        <button @click="showMore = !showMore" class="show-more-btn">
          <span>{{
            showMore
              ? "收起其他 UDP 連線"
              : `顯示其他 ${otherServers.length} 個 UDP 連線...`
          }}</span>
          <ChevronDown :class="{ rotated: showMore }" class="chevron-icon" />
        </button>
        <div class="divider"></div>
      </div>

      <!-- Other Servers List -->
      <Transition name="expand">
        <div
          v-if="showMore && otherServers.length > 0"
          class="other-servers-list"
        >
          <TransitionGroup name="server-list" tag="div" class="server-list">
            <div
              v-for="server in otherServers"
              :key="server.ip + server.port"
              class="server-card small"
              :class="{ 'is-game': server.is_game_server }"
            >
              <div class="server-row">
                <div class="server-mini-info">
                  <span class="badge-dot game"></span>
                  <span class="mini-ip">{{ server.ip }}</span>
                  <span class="mini-port">:{{ server.port }}</span>
                  <span class="mini-proto" v-if="server.country"
                    >({{ server.country }})</span
                  >
                </div>
                <div class="server-mini-stats">
                  <span class="rate-up"
                    ><ArrowUp class="tiny-icon" />{{
                      formatBytes(server.send_rate)
                    }}/s</span
                  >
                  <span class="rate-down"
                    ><ArrowDown class="tiny-icon" />{{
                      formatBytes(server.recv_rate)
                    }}/s</span
                  >
                </div>
                <button
                  @click="addToRoutes(server.ip)"
                  :disabled="addingRoute === server.ip"
                  class="icon-btn-add"
                  title="加入路由"
                >
                  <Plus
                    v-if="addingRoute !== server.ip"
                    class="btn-icon-tiny"
                  />
                  <Loader2 v-else class="btn-icon-tiny spin" />
                </button>
              </div>
            </div>
          </TransitionGroup>
        </div>
      </Transition>
    </div>

    <!-- Empty State -->
    <div v-else-if="isMonitoring" class="empty-state">
      <div class="empty-icon-container">
        <Search class="empty-icon" />
      </div>
      <p class="empty-text">正在掃描網路連接...</p>
      <p class="empty-subtext">請確保遊戲已啟動並進入對局</p>
    </div>

    <div v-else class="empty-state inactive">
      <div class="empty-icon-container">
        <Target class="empty-icon" />
      </div>
      <p class="empty-text">尚未啟動偵測</p>
      <p class="empty-subtext">點擊上方按鈕開始監控遊戲伺服器</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onUnmounted } from "vue";
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
  Search,
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
  statusMessage.value = "正在啟動監控...";
  statusType.value = "info";

  try {
    await invoke("start_monitoring", {
      processName: "TslGame.exe", // TODO: Use prop
    });

    isMonitoring.value = true;
    statusMessage.value = "監控已啟動，正在掃描連接...";
    statusType.value = "success";

    // Start polling for detected servers
    pollInterval = window.setInterval(fetchServers, 2000);

    // Also fetch immediately
    await fetchServers();
  } catch (error) {
    statusMessage.value = `啟動失敗: ${error}`;
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
    statusMessage.value = "監控已停止";
    statusType.value = "info";

    if (pollInterval !== null) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
    detectedServers.value = [];
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
      // Find the primary server's rate to make status more interesting?
      // Or just keep simple count
      const gameServers = servers.filter((s) => s.is_game_server);
      statusMessage.value = `已偵測 ${gameServers.length} 個遊戲節點，監測流量中...`;
      statusType.value = "success";
    }
  } catch (error) {
    console.error("Failed to fetch servers:", error);
  }
}

async function addToRoutes(ip: string) {
  addingRoute.value = ip;
  try {
    const result = await invoke<string>("add_detected_ip_to_routes", { ip });
    statusMessage.value = result;
    statusType.value = "success";
  } catch (error) {
    statusMessage.value = `加入路由失敗: ${error}`;
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

  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分鐘前`;
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

.server-detection-panel {
  font-family:
    "Noto Sans TC",
    -apple-system,
    sans-serif;
  background: linear-gradient(135deg, #0a0e27 0%, #1a1a2e 100%);
  border-radius: 16px;
  border: 1px solid rgba(139, 92, 246, 0.2);
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  background: linear-gradient(
    90deg,
    rgba(139, 92, 246, 0.1) 0%,
    rgba(219, 39, 119, 0.1) 100%
  );
  border-bottom: 1px solid rgba(139, 92, 246, 0.2);
}

.header-content {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #8b5cf6 0%, #db2777 100%);
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(139, 92, 246, 0.4);
}

.header-icon svg {
  width: 24px;
  height: 24px;
  color: white;
}

.header-icon svg.pulse {
  animation: pulse-glow 2s ease-in-out infinite;
}

@keyframes pulse-glow {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.panel-title {
  font-size: 20px;
  font-weight: 700;
  color: white;
  margin: 0;
  letter-spacing: -0.02em;
}

.panel-subtitle {
  font-size: 13px;
  color: rgba(255, 255, 255, 0.6);
  margin: 0;
}

.toggle-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 8px;
  color: white;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  font-family: "Noto Sans TC", sans-serif;
}

.toggle-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.15);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(139, 92, 246, 0.3);
}

.toggle-btn.active {
  background: linear-gradient(135deg, #8b5cf6 0%, #db2777 100%);
  border-color: transparent;
  box-shadow: 0 4px 16px rgba(139, 92, 246, 0.5);
}

.toggle-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toggle-btn .btn-icon {
  width: 18px;
  height: 18px;
}
.toggle-btn.loading .btn-icon {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.status-banner {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  font-size: 13px;
}

.status-banner.info {
  background: rgba(59, 130, 246, 0.1);
  color: #93c5fd;
}
.status-banner.success {
  background: rgba(34, 197, 94, 0.1);
  color: #86efac;
}
.status-banner.error {
  background: rgba(239, 68, 68, 0.1);
  color: #fca5a5;
}
.status-icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.servers-container {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* Primary Server Styles */
.primary-server-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 700;
  text-transform: uppercase;
  color: #f59e0b;
  letter-spacing: 0.05em;
}

.label-icon {
  width: 14px;
  height: 14px;
}

.server-card {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 16px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.server-card.primary {
  background: linear-gradient(
    135deg,
    rgba(245, 158, 11, 0.1) 0%,
    rgba(139, 92, 246, 0.05) 100%
  );
  border: 1px solid rgba(245, 158, 11, 0.3);
  box-shadow: 0 0 20px rgba(245, 158, 11, 0.1);
}

.server-card.primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 24px rgba(245, 158, 11, 0.2);
}

.server-main-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.server-ip-section {
  display: flex;
  align-items: center;
  gap: 12px;
}
.ip-icon {
  width: 24px;
  height: 24px;
  color: #f59e0b;
}
.ip-details {
  display: flex;
  align-items: baseline;
  gap: 4px;
}
.ip-address.big {
  font-family: "JetBrains Mono", monospace;
  font-size: 24px;
  font-weight: 700;
  color: white;
}
.ip-port {
  font-family: "JetBrains Mono", monospace;
  font-size: 14px;
  color: rgba(255, 255, 255, 0.6);
}

.server-location {
  display: flex;
  align-items: center;
  gap: 6px;
  color: rgba(255, 255, 255, 0.8);
  font-size: 14px;
}
.location-icon {
  width: 16px;
  height: 16px;
}

.server-stats.large {
  display: flex;
  gap: 32px;
  margin-bottom: 16px;
  padding: 16px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 12px;
}

.stat {
  display: flex;
  align-items: center;
  gap: 8px;
  font-family: "JetBrains Mono", monospace;
  font-size: 15px;
  color: rgba(255, 255, 255, 0.9);
}
.stat-icon {
  width: 18px;
  height: 18px;
}
.stat-icon.upload {
  color: #f59e0b;
}
.stat-icon.download {
  color: #10b981;
}

.server-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.add-route-btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 6px;
  color: white;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.add-route-btn.primary-btn {
  background: #f59e0b;
  color: black;
  border: none;
}

.add-route-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}
.add-route-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.server-type-badge {
  margin-bottom: 8px;
}
.badge {
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
}
.badge.game {
  background: #f59e0b;
  color: black;
}

/* Subheader / Divider */
.servers-subheader {
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 8px 0;
}

.divider {
  flex: 1;
  height: 1px;
  background: rgba(255, 255, 255, 0.1);
}
.show-more-btn {
  background: none;
  border: none;
  color: rgba(255, 255, 255, 0.5);
  font-size: 13px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: color 0.2s;
}
.show-more-btn:hover {
  color: white;
}
.chevron-icon {
  width: 14px;
  height: 14px;
  transition: transform 0.3s;
}
.chevron-icon.rotated {
  transform: rotate(180deg);
}

/* Other Servers Styles */
.other-servers-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.server-card.small {
  padding: 10px 16px;
}
.server-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.server-mini-info {
  display: flex;
  align-items: center;
  gap: 8px;
}
.mini-ip {
  font-family: "JetBrains Mono", monospace;
  color: white;
  font-size: 14px;
}
.mini-port {
  font-family: "JetBrains Mono", monospace;
  color: rgba(255, 255, 255, 0.5);
  font-size: 12px;
}
.mini-proto {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.4);
}

.badge-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}
.badge-dot.game {
  background: #f59e0b;
  box-shadow: 0 0 8px #f59e0b;
}
.badge-dot.lobby {
  background: rgba(255, 255, 255, 0.3);
}

.server-mini-stats {
  display: flex;
  gap: 12px;
  font-family: "JetBrains Mono";
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
}
.tiny-icon {
  width: 12px;
  height: 12px;
  margin-right: 2px;
  vertical-align: middle;
}
.btn-icon-tiny {
  width: 14px;
  height: 14px;
}

.icon-btn-add {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  width: 28px;
  height: 28px;
  border-radius: 6px;
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
.icon-btn-add:hover {
  background: #f59e0b;
  color: black;
}

/* Transitions */
.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  max-height: 500px;
  opacity: 1;
  overflow: hidden;
}

.expand-enter-from,
.expand-leave-to {
  max-height: 0;
  opacity: 0;
}

.server-list-enter-active,
.server-list-leave-active {
  transition: all 0.3s ease;
}
.server-list-enter-from {
  opacity: 0;
  transform: translateX(-10px);
}
.server-list-leave-to {
  opacity: 0;
  transform: translateX(10px);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 24px;
  text-align: center;
}
.empty-icon-container {
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(
    135deg,
    rgba(139, 92, 246, 0.2) 0%,
    rgba(219, 39, 119, 0.2) 100%
  );
  border-radius: 20px;
  margin-bottom: 24px;
}
.empty-icon {
  width: 40px;
  height: 40px;
  color: #8b5cf6;
  animation: float 3s ease-in-out infinite;
}
@keyframes float {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}
.empty-text {
  font-size: 16px;
  font-weight: 600;
  color: white;
  margin: 0 0 8px 0;
}
.empty-subtext {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.6);
  margin: 0;
}
</style>
