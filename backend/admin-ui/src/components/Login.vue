<script setup lang="ts">
import { ref } from "vue";
import { useAuth } from "../composables/useAuth";

const { login } = useAuth();

const username = ref("");
const password = ref("");
const error = ref("");
const loading = ref(false);

async function handleSubmit() {
  if (!username.value || !password.value) return;

  loading.value = true;
  error.value = "";

  const success = await login(username.value, password.value);

  if (!success) {
    error.value = "Invalid username or password";
  }

  loading.value = false;
}
</script>

<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <h1 class="title">NigPing Admin</h1>
        <p class="subtitle">Sign in to manage servers</p>
      </div>

      <form @submit.prevent="handleSubmit" class="login-form">
        <div class="form-group">
          <label for="username">Username</label>
          <input
            id="username"
            v-model="username"
            type="text"
            placeholder="Enter username"
            required
            class="form-input"
          />
        </div>

        <div class="form-group">
          <label for="password">Password</label>
          <input
            id="password"
            v-model="password"
            type="password"
            placeholder="Enter password"
            required
            class="form-input"
          />
        </div>

        <div v-if="error" class="error-msg">
          {{ error }}
        </div>

        <button type="submit" :disabled="loading" class="btn-submit">
          {{ loading ? "Signing in..." : "Sign In" }}
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(to bottom, #fafaf9, #f5f5f4);
  padding: 1rem;
}

.login-card {
  background: white;
  padding: 2.5rem;
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.05);
  width: 100%;
  max-width: 400px;
  border: 1px solid #e7e5e4;
}

.login-header {
  text-align: center;
  margin-bottom: 2rem;
}

.title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 700;
  color: #292524;
}

.subtitle {
  margin: 0.5rem 0 0;
  color: #78716c;
  font-size: 0.9375rem;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.875rem;
  font-weight: 600;
  color: #57534e;
}

.form-input {
  padding: 0.75rem;
  border: 1.5px solid #d6d3d1;
  border-radius: 8px;
  font-size: 0.9375rem;
  transition: all 0.2s;
  outline: none;
}

.form-input:focus {
  border-color: #78716c;
  box-shadow: 0 0 0 3px rgba(120, 113, 108, 0.1);
}

.error-msg {
  color: #ef4444;
  font-size: 0.875rem;
  text-align: center;
  background: #fee2e2;
  padding: 0.5rem;
  border-radius: 6px;
}

.btn-submit {
  background: #292524;
  color: white;
  padding: 0.875rem;
  border: none;
  border-radius: 8px;
  font-size: 0.9375rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  margin-top: 0.5rem;
}

.btn-submit:hover:not(:disabled) {
  background: #1c1917;
  transform: translateY(-1px);
}

.btn-submit:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}
</style>
