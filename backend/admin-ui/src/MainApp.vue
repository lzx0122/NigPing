<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useServers } from "./composables/useServers";
import ServerForm from "./components/ServerForm.vue";
import ServerList from "./components/ServerList.vue";
import DeleteConfirm from "./components/DeleteConfirm.vue";
import GameRangeManager from "./components/GameRangeManager.vue";
import UserManagement from "./components/UserManagement.vue";
import Login from "./components/Login.vue";
import type { Server } from "./types/server";
import { useAuth } from "./composables/useAuth";

const { isAuthenticated, logout } = useAuth();

const {
  filteredServers,
  stats,
  loading,
  error,
  searchQuery,
  fetchServers,
  addServer,
  updateServer,
  deleteServer,
} = useServers();

const currentTab = ref<"servers" | "ranges" | "users">("servers");
const showAddForm = ref(false);
const editingServer = ref<Server | null>(null);
const deletingServerIp = ref<string | null>(null);
const selectedGameId = ref("pubg");

onMounted(() => {
  fetchServers();
});

async function handleAddServer(ip: string, region: string) {
  const success = await addServer(ip, region);
  if (success) {
    showAddForm.value = false;
  }
}

async function handleUpdateServer(ip: string, region: string) {
  if (!editingServer.value) return;
  const success = await updateServer(editingServer.value.ip, ip, region);
  if (success) {
    editingServer.value = null;
  }
}

function handleEditServer(server: Server) {
  editingServer.value = server;
  showAddForm.value = false;
}

function handleDeleteClick(ip: string) {
  deletingServerIp.value = ip;
}

async function confirmDelete() {
  if (!deletingServerIp.value) return;
  await deleteServer(deletingServerIp.value);
  deletingServerIp.value = null;
}

function toggleAddForm() {
  showAddForm.value = !showAddForm.value;
  editingServer.value = null;
}

function handleLogout() {
  logout();
}
</script>

<template>
  <Login v-if="!isAuthenticated" />
  <div v-else class="app">
    <header class="header">
      <div class="header-content">
        <div class="header-left">
          <h1 class="title">NigPing Server Manager</h1>
          <p class="subtitle">Manage your server infrastructure</p>
        </div>
        <div class="flex gap-4">
          <button
            :class="['tab-btn', currentTab === 'servers' ? 'active' : '']"
            @click="currentTab = 'servers'"
          >
            Servers
          </button>
          <button
            :class="['tab-btn', currentTab === 'ranges' ? 'active' : '']"
            @click="currentTab = 'ranges'"
          >
            Game Ranges
          </button>
          <button
            :class="['tab-btn', currentTab === 'users' ? 'active' : '']"
            @click="currentTab = 'users'"
          >
            Users
          </button>
        </div>
        <button
          v-if="currentTab === 'servers'"
          class="btn-add"
          @click="toggleAddForm"
        >
          + Add Server
        </button>
        <button class="btn-logout" @click="handleLogout" title="Logout">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
            <polyline points="16 17 21 12 16 7"></polyline>
            <line x1="21" y1="12" x2="9" y2="12"></line>
          </svg>
        </button>
      </div>
    </header>

    <main class="main">
      <div class="container">
        <!-- Users Tab -->
        <div v-if="currentTab === 'users'">
          <UserManagement />
        </div>

        <!-- Game Ranges Tab -->
        <div v-else-if="currentTab === 'ranges'" class="space-y-6">
          <div class="game-selector-card">
            <label class="form-label">Select Game</label>
            <select v-model="selectedGameId" class="form-select">
              <option value="pubg">PUBG</option>
              <!-- Add more games here as needed -->
            </select>
          </div>

          <div class="range-manager-card">
            <GameRangeManager :gameId="selectedGameId" />
          </div>
        </div>

        <!-- Servers Tab -->
        <div v-else>
          <!-- Stats Section -->
          <div class="stats-grid">
            <div class="stat-card">
              <div class="stat-icon">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
                  <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
                  <line x1="6" y1="6" x2="6.01" y2="6"></line>
                  <line x1="6" y1="18" x2="6.01" y2="18"></line>
                </svg>
              </div>
              <div class="stat-content">
                <p class="stat-label">Total Servers</p>
                <p class="stat-value">{{ stats.total }}</p>
              </div>
            </div>

            <div class="stat-card">
              <div class="stat-icon">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="24"
                  height="24"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <circle cx="12" cy="12" r="10"></circle>
                  <line x1="2" y1="12" x2="22" y2="12"></line>
                  <path
                    d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1 4-10 15.3 15.3 0 0 1 4-10z"
                  ></path>
                </svg>
              </div>
              <div class="stat-content">
                <p class="stat-label">Regions</p>
                <p class="stat-value">
                  {{ Object.keys(stats.byRegion).length }}
                </p>
              </div>
            </div>
          </div>

          <!-- Error Message -->
          <div v-if="error" class="error-banner">
            <span>{{ error }}</span>
          </div>

          <!-- Add/Edit Form -->
          <ServerForm
            v-if="showAddForm"
            mode="add"
            @submit="handleAddServer"
            @cancel="showAddForm = false"
          />

          <ServerForm
            v-if="editingServer"
            mode="edit"
            :initial-ip="editingServer.ip"
            :initial-region="editingServer.region"
            @submit="handleUpdateServer"
            @cancel="editingServer = null"
          />

          <!-- Search Bar -->
          <div class="search-section">
            <div class="search-bar">
              <input
                v-model="searchQuery"
                type="text"
                placeholder="Search by IP or region..."
              />
            </div>
          </div>

          <!-- Server List -->
          <ServerList
            :servers="filteredServers"
            :loading="loading"
            @edit="handleEditServer"
            @delete="handleDeleteClick"
          />
        </div>
      </div>
    </main>

    <!-- Delete Confirmation Modal -->
    <DeleteConfirm
      :show="!!deletingServerIp"
      :server-ip="deletingServerIp || ''"
      @confirm="confirmDelete"
      @cancel="deletingServerIp = null"
    />
  </div>
</template>

<style scoped>
.app {
  min-height: 100vh;
  background: linear-gradient(to bottom, #fafaf9, #f5f5f4);
}

.header {
  background: #ffffff;
  border-bottom: 1px solid #e7e5e4;
  padding: 2rem 0;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 2rem;
}

.header-left {
  flex: 1;
}

.title {
  margin: 0;
  font-size: 1.875rem;
  font-weight: 700;
  color: #292524;
  letter-spacing: -0.025em;
}

.subtitle {
  margin: 0.375rem 0 0 0;
  font-size: 0.9375rem;
  color: #78716c;
}

.btn-add {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: #292524;
  color: #fafaf9;
  border: none;
  border-radius: 10px;
  font-size: 0.9375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-add:hover {
  background: #1c1917;
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(41, 37, 36, 0.2);
}

.main {
  padding: 2.5rem 0;
}

.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 1.25rem;
  margin-bottom: 2rem;
}

.stat-card {
  background: #ffffff;
  border: 1px solid #e7e5e4;
  border-radius: 12px;
  padding: 1.5rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all 0.2s ease;
}

.stat-card:hover {
  border-color: #d6d3d1;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.stat-icon {
  width: 48px;
  height: 48px;
  background: #fafaf9;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #78716c;
}

.stat-content {
  flex: 1;
}

.stat-label {
  margin: 0;
  font-size: 0.8125rem;
  font-weight: 500;
  color: #78716c;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.stat-value {
  margin: 0.25rem 0 0 0;
  font-size: 1.875rem;
  font-weight: 700;
  color: #292524;
}

.error-banner {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 1.25rem;
  background: #fee2e2;
  border: 1px solid #fecaca;
  border-radius: 10px;
  color: #991b1b;
  margin-bottom: 2rem;
}

.search-section {
  margin-bottom: 1.5rem;
}

.search-bar {
  position: relative;
  max-width: 400px;
}

.search-bar svg {
  position: absolute;
  left: 1rem;
  top: 50%;
  transform: translateY(-50%);
  color: #a8a29e;
  pointer-events: none;
}

.search-bar input {
  width: 100%;
  padding: 0.75rem 1rem 0.75rem 3rem;
  border: 1.5px solid #e7e5e4;
  border-radius: 10px;
  font-size: 0.9375rem;
  color: #292524;
  background: #ffffff;
  transition: all 0.2s ease;
}

.search-bar input:focus {
  outline: none;
  border-color: #78716c;
  box-shadow: 0 0 0 3px rgba(120, 113, 108, 0.1);
}

.search-bar input::placeholder {
  color: #a8a29e;
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .btn-add {
    width: 100%;
    justify-content: center;
  }

  .title {
    font-size: 1.5rem;
  }
}

.tab-btn {
  padding: 0.5rem 1rem;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: #78716c;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.tab-btn:hover {
  background: #f5f5f4;
  color: #292524;
}

.tab-btn.active {
  background: #e7e5e4;
  color: #292524;
  font-weight: 600;
}

.game-selector-card {
  background: #ffffff;
  padding: 1.5rem;
  border-radius: 12px;
  border: 1px solid #e7e5e4;
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
}

.range-manager-card {
  background: #ffffff;
  padding: 1.5rem;
  border-radius: 12px;
  border: 1px solid #e7e5e4;
}

.form-label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  color: #57534e;
  margin-bottom: 0;
  white-space: nowrap;
}

.form-select {
  display: block;
  width: 100%;
  max-width: 20rem;
  padding: 0.625rem 0.875rem;
  border: 1.5px solid #d6d3d1;
  border-radius: 8px;
  font-size: 0.9375rem;
  color: #292524;
  background-color: #ffffff;
  transition: all 0.2s ease;
}

.form-select:focus {
  outline: none;
  border-color: #78716c;
  box-shadow: 0 0 0 3px rgba(120, 113, 108, 0.1);
}

.btn-logout {
  background: transparent;
  border: 1px solid #e7e5e4;
  color: #78716c;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-logout:hover {
  background: #fee2e2;
  color: #ef4444;
  border-color: #fecaca;
}
</style>
