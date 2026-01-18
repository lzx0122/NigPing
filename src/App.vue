<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 狀態管理
const status = ref("未連線");
const isConnected = ref(false);
const isLoading = ref(false);

// ==========================================
// 👇 請在這裡填入你 GCP 的 WireGuard 資料 👇
// ==========================================
const PRIVATE_KEY = "8J3JPRxASoiS1pepU4NE3VoRUB8ILdpSBVKZrXt1uXs="; // 你的私鑰
const PUBLIC_KEY = "J8j5OOO9qtR8eI+GSw+TBttF3scLv1aiUeoLoMu8B2w="; // GCP Server 的公鑰
const ENDPOINT = "34.80.46.250:51820"; // GCP 的外部 IP : Port
const CLIENT_IP = "10.8.0.2"; // 你的內網 IP (Address 去掉 /32)
// ==========================================

// 產生設定檔內容 (動態生成)
const getWgConfig = () => `
[Interface]
PrivateKey = ${PRIVATE_KEY}
Address = 10.8.0.2/24
DNS = 1.1.1.1
MTU = 1280

[Peer]
PublicKey = ${PUBLIC_KEY}
PresharedKey = JbLLJPvjfXhykHg8mDrNdonHhNTlAYZNh9v3u8bbNzI=
AllowedIPs = 0.0.0.0/0, ::/0
Endpoint = ${ENDPOINT}

`;

// 🔵 啟動連線
const handleConnect = async () => {
  if (PRIVATE_KEY.includes("請貼上")) {
    alert("請先在 App.vue 程式碼中填入正確的 GCP Key！");
    return;
  }

  isLoading.value = true;
  status.value = "🚀 正在啟動引擎...";

  try {
    // 呼叫 Rust 後端
    const msg = await invoke("connect_korea", {
      configContent: getWgConfig(),
      ipv4Address: CLIENT_IP,
    });

    status.value = "✅ " + msg; // 顯示 "韓服連線成功"
    isConnected.value = true;
  } catch (error) {
    console.error(error);
    status.value = "❌ 連線失敗: " + error;
    alert("連線失敗，請檢查 Log 或管理員權限");
  } finally {
    isLoading.value = false;
  }
};

// 🔴 斷開連線
const handleDisconnect = async () => {
  isLoading.value = true;
  status.value = "🛑 正在中斷...";

  try {
    const msg = await invoke("disconnect_vpn");
    status.value = "💤 " + msg;
    isConnected.value = false;
  } catch (error) {
    status.value = "❌ 斷線失敗: " + error;
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <div class="container">
    <h1>NigPing 加速器 🇰🇷</h1>

    <div class="card">
      <div class="status-indicator" :class="{ active: isConnected }">
        <div class="dot" :class="isConnected ? 'green' : 'red'"></div>
        <span>目前狀態: {{ status }}</span>
      </div>

      <div class="button-group">
        <button
          @click="handleConnect"
          :disabled="isConnected || isLoading"
          class="btn-connect"
        >
          {{ isLoading && !isConnected ? "啟動中..." : "啟動韓服加速" }}
        </button>

        <button
          @click="handleDisconnect"
          :disabled="!isConnected || isLoading"
          class="btn-disconnect"
        >
          斷開連線
        </button>
      </div>

      <p class="note">
        {{
          isConnected
            ? "🔥 已連線至首爾伺服器，請開啟 PUBG 測試"
            : "⚠️ 請確保以「系統管理員身分」執行此程式"
        }}
      </p>
    </div>
  </div>
</template>

<style scoped>
/* 這裡直接寫入樣式，不需要額外的 css 檔案 */
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  background-color: #1a1a1a;
  color: white;
  font-family: "Segoe UI", sans-serif;
}

h1 {
  margin-bottom: 2rem;
  color: #ff9800;
}

.card {
  background-color: #2a2a2a;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  text-align: center;
  width: 300px;
}

.status-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.5rem;
  font-weight: bold;
  padding: 10px;
  background: #333;
  border-radius: 8px;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  margin-right: 10px;
}

.dot.green {
  background-color: #00ff00;
  box-shadow: 0 0 10px #00ff00;
}
.dot.red {
  background-color: #ff4444;
}

.button-group {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

button {
  padding: 12px 24px;
  font-size: 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  font-weight: bold;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-connect {
  background-color: #007acc;
  color: white;
}

.btn-connect:hover:not(:disabled) {
  background-color: #005fa3;
  transform: translateY(-2px);
}

.btn-disconnect {
  background-color: #d32f2f;
  color: white;
}

.btn-disconnect:hover:not(:disabled) {
  background-color: #b71c1c;
}

.note {
  margin-top: 1.5rem;
  font-size: 0.8rem;
  color: #888;
}
</style>
