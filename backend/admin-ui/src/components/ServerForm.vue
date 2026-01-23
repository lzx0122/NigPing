<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  mode: "add" | "edit";
  initialIp?: string;
  initialRegion?: string;
}>();

const emit = defineEmits<{
  submit: [ip: string, region: string];
  cancel: [];
}>();

const ip = ref(props.initialIp || "");
const region = ref(props.initialRegion || "");
const errors = ref<{ ip?: string; region?: string }>({});

function validateForm() {
  errors.value = {};

  if (!ip.value.trim()) {
    errors.value.ip = "IP address is required";
  } else if (!/^(\d{1,3}\.){3}\d{1,3}$/.test(ip.value)) {
    errors.value.ip = "Invalid IP address format";
  }

  if (!region.value.trim()) {
    errors.value.region = "Region is required";
  }

  return Object.keys(errors.value).length === 0;
}

function handleSubmit() {
  if (validateForm()) {
    emit("submit", ip.value.trim(), region.value.trim());
  }
}

function handleCancel() {
  emit("cancel");
}
</script>

<template>
  <div class="form-container">
    <h3 class="form-title">
      {{ mode === "add" ? "Add New Server" : "Edit Server" }}
    </h3>

    <form @submit.prevent="handleSubmit">
      <div class="form-group">
        <label for="ip">IP Address</label>
        <input
          id="ip"
          v-model="ip"
          type="text"
          placeholder="192.168.1.1"
          :class="{ error: errors.ip }"
        />
        <span v-if="errors.ip" class="error-message">{{ errors.ip }}</span>
      </div>

      <div class="form-group">
        <label for="region">Region</label>
        <input
          id="region"
          v-model="region"
          type="text"
          placeholder="Taiwan"
          :class="{ error: errors.region }"
        />
        <span v-if="errors.region" class="error-message">{{
          errors.region
        }}</span>
      </div>

      <div class="form-actions">
        <button type="button" class="btn btn-secondary" @click="handleCancel">
          Cancel
        </button>
        <button type="submit" class="btn btn-primary">
          {{ mode === "add" ? "Add Server" : "Update Server" }}
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
.form-container {
  background: #fafaf9;
  border: 1px solid #e7e5e4;
  border-radius: 12px;
  padding: 1.75rem;
  margin-bottom: 2rem;
}

.form-title {
  margin: 0 0 1.5rem 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #292524;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: #57534e;
  letter-spacing: 0.01em;
}

.form-group input {
  width: 100%;
  padding: 0.625rem 0.875rem;
  border: 1.5px solid #d6d3d1;
  border-radius: 8px;
  font-size: 0.9375rem;
  color: #292524;
  background: #ffffff;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.form-group input:focus {
  outline: none;
  border-color: #78716c;
  box-shadow: 0 0 0 3px rgba(120, 113, 108, 0.1);
}

.form-group input.error {
  border-color: #dc2626;
}

.error-message {
  display: block;
  margin-top: 0.375rem;
  font-size: 0.8125rem;
  color: #dc2626;
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  margin-top: 1.75rem;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 8px;
  font-size: 0.9375rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
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

.btn-secondary {
  background: #ffffff;
  color: #57534e;
  border: 1.5px solid #d6d3d1;
}

.btn-secondary:hover {
  background: #f5f5f4;
  border-color: #a8a29e;
}
</style>
