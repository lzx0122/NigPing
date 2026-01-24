<script setup lang="ts">
import { ref, onMounted, watch } from "vue";

const props = defineProps<{
  gameId: string;
}>();

const ranges = ref<string[]>([]);
const newRange = ref("");
const loading = ref(false);
const error = ref("");
const success = ref("");

const fetchRanges = async () => {
  loading.value = true;
  error.value = "";
  try {
    const res = await fetch(`/api/games/${props.gameId}/ranges`);
    if (!res.ok) throw new Error("Failed to fetch ranges");
    ranges.value = await res.json();
  } catch (e: any) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
};

const addRange = async () => {
  if (!newRange.value) return;

  // Basic validation (can be improved)
  if (!newRange.value.includes("/")) {
    error.value = "Invalid CIDR format (e.g., 1.2.3.0/24)";
    return;
  }

  loading.value = true;
  error.value = "";
  success.value = "";

  try {
    const res = await fetch(`/api/games/${props.gameId}/ranges`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ ipRange: newRange.value }),
    });

    if (!res.ok) {
      const data = await res.json();
      throw new Error(data.message || "Failed to add range");
    }

    success.value = `Added ${newRange.value}`;
    newRange.value = "";
    await fetchRanges();
  } catch (e: any) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
};

const deleteRange = async (range: string) => {
  if (!confirm(`Are you sure you want to delete ${range}?`)) return;

  loading.value = true;
  error.value = "";
  success.value = "";

  try {
    // Pass range as query param to avoid URL encoding issues with "/"
    const res = await fetch(
      `/api/games/${props.gameId}/ranges?range=${encodeURIComponent(range)}`,
      {
        method: "DELETE",
      },
    );

    if (!res.ok) throw new Error("Failed to delete range");

    success.value = `Deleted ${range}`;
    await fetchRanges();
  } catch (e: any) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
};

onMounted(fetchRanges);

watch(() => props.gameId, fetchRanges);
</script>

<template>
  <div class="range-manager">
    <div class="range-header">
      <div class="flex items-center gap-3">
        <h2 class="title">Game: {{ gameId }}</h2>
        <span v-if="ranges.length > 0" class="count-badge"
          >{{ ranges.length }} ranges</span
        >
      </div>
      <button @click="fetchRanges" class="btn-refresh">Refresh</button>
    </div>

    <!-- Feedback -->
    <div v-if="error" class="banner banner-error">
      {{ error }}
    </div>
    <div v-if="success" class="banner banner-success">
      {{ success }}
    </div>

    <!-- Add Form -->
    <div class="add-form">
      <div class="form-group">
        <label>Add New IP Range</label>
        <div class="input-row">
          <input
            v-model="newRange"
            type="text"
            placeholder="e.g. 13.124.0.0/16"
            @keyup.enter="addRange"
          />
          <button
            @click="addRange"
            :disabled="loading || !newRange"
            class="btn btn-primary"
          >
            {{ loading ? "Adding..." : "Add Range" }}
          </button>
        </div>
      </div>
    </div>

    <!-- List -->
    <div class="range-list">
      <div v-if="ranges.length === 0" class="empty-state">
        No IP ranges found for this game.
      </div>
      <ul v-else class="list">
        <li v-for="range in ranges" :key="range" class="list-item">
          <div class="range-info">
            <span class="range-value">{{ range }}</span>
            <span class="badge">CIDR</span>
          </div>
          <button
            @click="deleteRange(range)"
            class="btn-delete"
            :disabled="loading"
          >
            Delete
          </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<style>
/* Skill-aligned styling: Clean, consistent, and polished */
.range-manager {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  font-family: inherit; /* Inherit DM Sans */
}

.range-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.count-badge {
  background: #f5f5f4;
  color: #78716c;
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.875rem;
  font-weight: 600;
  border: 1px solid #e7e5e4;
}

.title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: #292524;
  letter-spacing: -0.025em;
}

.btn-refresh {
  background: white;
  border: 1px solid #e7e5e4;
  color: #57534e;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
}

.btn-refresh:hover {
  background: #fafaf9;
  color: #292524;
  border-color: #d6d3d1;
  transform: translateY(-1px);
}

.banner {
  padding: 1rem 1.25rem;
  border-radius: 8px;
  font-size: 0.9375rem;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  animation: fadeIn 0.3s ease-out;
}

.banner-error {
  background: #fee2e2;
  color: #991b1b;
  border: 1px solid #fecaca;
}

.banner-success {
  background: #dcfce7;
  color: #166534;
  border: 1px solid #bbf7d0;
}

.add-form {
  background: #ffffff;
  border: 1px solid #e7e5e4;
  border-radius: 12px;
  padding: 1.5rem;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.form-group label {
  display: block;
  margin-bottom: 0.75rem;
  font-size: 0.9375rem;
  font-weight: 600;
  color: #44403c;
}

.input-row {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.add-form input {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 1.5px solid #e7e5e4;
  border-radius: 10px;
  font-size: 0.9375rem;
  color: #292524;
  background: #ffffff;
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  font-family: monospace; /* For IP addresses */
}

.add-form input:focus {
  outline: none;
  border-color: #57534e;
  box-shadow: 0 0 0 4px rgba(87, 83, 78, 0.1);
}

.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 10px;
  font-size: 0.9375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  white-space: nowrap;
}

.btn-primary {
  background: #292524;
  color: #fafaf9;
}

.btn-primary:hover {
  background: #1c1917;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(41, 37, 36, 0.15);
}

.btn-primary:active {
  transform: translateY(0);
}

.btn-primary:disabled {
  background: #a8a29e;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.range-list {
  background: #ffffff;
  border: 1px solid #e7e5e4;
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.empty-state {
  padding: 4rem 2rem;
  text-align: center;
  color: #a8a29e;
  font-size: 0.9375rem;
  font-style: italic;
}

.list {
  list-style: none;
  margin: 0;
  padding: 0;
}

.list-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid #f5f5f4;
  transition: background 0.15s ease;
}

.list-item:last-child {
  border-bottom: none;
}

.list-item:hover {
  background: #fafaf9;
}

.range-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.range-value {
  font-family: "DM Mono", "JetBrains Mono", monospace;
  font-size: 1rem;
  color: #292524;
  font-weight: 500;
  background: #f5f5f4;
  padding: 0.25rem 0.5rem;
  border-radius: 6px;
}

.badge {
  padding: 0.25rem 0.625rem;
  border-radius: 20px;
  background: #f5f5f4;
  color: #78716c;
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border: 1px solid #e7e5e4;
}

.btn-delete {
  background: transparent;
  border: none;
  color: #ef4444;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  padding: 0.5rem 1rem;
  border-radius: 8px;
  transition: all 0.2s cubic-bezier(0.16, 1, 0.3, 1);
  opacity: 0.8;
}

.btn-delete:hover {
  background: #fee2e2;
  color: #dc2626;
  opacity: 1;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
