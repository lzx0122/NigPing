import { ref, computed } from "vue";

const token = ref<string | null>(localStorage.getItem("token"));
const username = ref<string | null>(localStorage.getItem("username"));

// API URL from environment variable, fallback to localhost
const API_URL = import.meta.env.VITE_API_URL || "http://localhost:3000";

export function useAuth() {
  const isAuthenticated = computed(() => !!token.value);

  async function login(user: string, pass: string): Promise<boolean> {
    console.log("🔐 [Login] Starting login process...");
    console.log("🔐 [Login] Username:", user);
    console.log("🔐 [Login] API URL:", API_URL);
    console.log("🔐 [Login] Full endpoint:", `${API_URL}/api/auth/login`);

    try {
      console.log("🔐 [Login] Sending fetch request...");
      const response = await fetch(`${API_URL}/api/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username: user, password: pass }),
      });

      console.log("🔐 [Login] Response status:", response.status);
      console.log("🔐 [Login] Response ok:", response.ok);
      console.log(
        "🔐 [Login] Response headers:",
        Object.fromEntries(response.headers.entries()),
      );

      if (!response.ok) {
        const errorText = await response.text();
        console.error("❌ [Login] Login failed - Response not OK");
        console.error("❌ [Login] Error response:", errorText);
        throw new Error("Invalid credentials");
      }

      console.log("🔐 [Login] Parsing response JSON...");
      const data = await response.json();
      console.log("🔐 [Login] Response data:", {
        ...data,
        token: data.token ? `${data.token.substring(0, 20)}...` : "missing",
      });

      token.value = data.token;
      username.value = data.username;

      localStorage.setItem("token", data.token);
      localStorage.setItem("username", data.username);

      console.log("✅ [Login] Login successful!");
      console.log("✅ [Login] Token saved to localStorage");
      console.log("✅ [Login] Username:", data.username);

      return true;
    } catch (e) {
      console.error("❌ [Login] Login error caught:", e);
      console.error(
        "❌ [Login] Error type:",
        e instanceof Error ? e.constructor.name : typeof e,
      );
      console.error(
        "❌ [Login] Error message:",
        e instanceof Error ? e.message : String(e),
      );
      console.error(
        "❌ [Login] Error stack:",
        e instanceof Error ? e.stack : "N/A",
      );
      return false;
    }
  }

  function logout() {
    token.value = null;
    username.value = null;
    localStorage.removeItem("token");
    localStorage.removeItem("username");
  }

  function getAuthHeaders(): Record<string, string> {
    return token.value ? { Authorization: `Bearer ${token.value}` } : {};
  }

  return {
    token,
    username,
    isAuthenticated,
    login,
    logout,
    getAuthHeaders,
  };
}
