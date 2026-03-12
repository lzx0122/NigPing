import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { setItem, getItem, removeItem } from "../lib/store";

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

export const useAuthStore = defineStore("auth", () => {
  const token = ref<string | null>(null);
  const username = ref<string | null>(null);

  const isAuthenticated = computed(() => !!token.value);

  async function loadState() {
    token.value = (await getItem<string>("auth_token")) || null;
    username.value = (await getItem<string>("username")) || null;
  }

  async function login(user: string, pass: string): Promise<boolean> {
    try {
      const response = await fetch(`${API_URL}/api/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: user, password: pass }),
      });

      if (!response.ok) {
        throw new Error("Invalid credentials");
      }

      const data = await response.json();

      token.value = data.token;
      username.value = data.username;

      await setItem("auth_token", data.token);
      await setItem("username", data.username);

      return true;
    } catch (e) {
      console.error("Login Error:", e);
      return false;
    }
  }

  async function logout() {
    token.value = null;
    username.value = null;
    await removeItem("auth_token");
    await removeItem("username");
  }

  function getAuthHeaders(): Record<string, string> {
    return token.value ? { Authorization: `Bearer ${token.value}` } : {};
  }

  return {
    token,
    username,
    isAuthenticated,
    loadState,
    login,
    logout,
    getAuthHeaders,
  };
});
