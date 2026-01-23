<script setup lang="ts">
import type { Server } from "../types/server";

defineProps<{
  servers: Server[];
  loading: boolean;
}>();

const emit = defineEmits<{
  edit: [server: Server];
  delete: [ip: string];
}>();

function formatDate(dateString: string) {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}
</script>

<template>
  <div class="server-list">
    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading servers...</p>
    </div>

    <div v-else-if="servers.length === 0" class="empty-state">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="48"
        height="48"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <rect x="2" y="2" width="20" height="8" rx="2" ry="2"></rect>
        <rect x="2" y="14" width="20" height="8" rx="2" ry="2"></rect>
        <line x1="6" y1="6" x2="6.01" y2="6"></line>
        <line x1="6" y1="18" x2="6.01" y2="18"></line>
      </svg>
      <p>No servers found</p>
      <span>Add your first server to get started</span>
    </div>

    <div v-else class="table-container">
      <table>
        <thead>
          <tr>
            <th>IP Address</th>
            <th>Region</th>
            <th>Added At</th>
            <th class="actions-header">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="server in servers" :key="server.ip" class="server-row">
            <td class="ip-cell">
              <code>{{ server.ip }}</code>
            </td>
            <td class="region-cell">
              <span class="region-badge">{{ server.region }}</span>
            </td>
            <td class="date-cell">{{ formatDate(server.addedAt) }}</td>
            <td class="actions-cell">
              <button
                class="action-btn edit-btn"
                @click="emit('edit', server)"
                title="Edit server"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <path
                    d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
                  ></path>
                  <path
                    d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
                  ></path>
                </svg>
              </button>
              <button
                class="action-btn delete-btn"
                @click="emit('delete', server.ip)"
                title="Delete server"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <polyline points="3 6 5 6 21 6"></polyline>
                  <path
                    d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                  ></path>
                </svg>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.server-list {
  min-height: 300px;
}

.loading-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  color: #78716c;
}

.loading-state p,
.empty-state p {
  margin: 1rem 0 0.25rem 0;
  font-size: 1rem;
  font-weight: 500;
  color: #57534e;
}

.empty-state span {
  font-size: 0.875rem;
  color: #a8a29e;
}

.empty-state svg {
  color: #d6d3d1;
  margin-bottom: 0.5rem;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #e7e5e4;
  border-top-color: #78716c;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.table-container {
  overflow-x: auto;
  border-radius: 12px;
  border: 1px solid #e7e5e4;
  background: #ffffff;
}

table {
  width: 100%;
  border-collapse: collapse;
}

thead {
  background: #fafaf9;
  border-bottom: 1px solid #e7e5e4;
}

th {
  text-align: left;
  padding: 1rem 1.25rem;
  font-size: 0.8125rem;
  font-weight: 600;
  color: #78716c;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.actions-header {
  text-align: right;
}

tbody tr {
  border-bottom: 1px solid #f5f5f4;
  transition: background-color 0.15s ease;
}

tbody tr:last-child {
  border-bottom: none;
}

tbody tr:hover {
  background: #fafaf9;
}

td {
  padding: 1rem 1.25rem;
  color: #57534e;
}

.ip-cell code {
  font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
  font-size: 0.9375rem;
  color: #292524;
  background: #f5f5f4;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-weight: 500;
}

.region-badge {
  display: inline-block;
  padding: 0.375rem 0.75rem;
  background: #e7e5e4;
  color: #44403c;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
}

.date-cell {
  font-size: 0.875rem;
  color: #78716c;
}

.actions-cell {
  text-align: right;
}

.action-btn {
  padding: 0.5rem;
  border: none;
  background: transparent;
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s ease;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  margin-left: 0.375rem;
}

.edit-btn {
  color: #78716c;
}

.edit-btn:hover {
  background: #f5f5f4;
  color: #292524;
}

.delete-btn {
  color: #dc2626;
}

.delete-btn:hover {
  background: #fee2e2;
  color: #b91c1c;
}

@media (max-width: 768px) {
  th,
  td {
    padding: 0.75rem 1rem;
  }

  .date-cell {
    display: none;
  }
}
</style>
