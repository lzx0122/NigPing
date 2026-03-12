// Vue Router configuration
import type { RouteLocationNormalized, NavigationGuardNext } from "vue-router";
import { createRouter, createWebHashHistory } from "vue-router";
import { useAuthStore } from "../stores/authStore";
import LoginView from "../components/Login.vue";

const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/login",
      name: "login",
      component: LoginView,
      meta: { requiresAuth: false },
    },
    {
      path: "/",
      name: "home",
      // To be extracted from App.vue
      component: () => import("../views/DashboardView.vue"),
      meta: { requiresAuth: true },
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/SettingsView.vue"),
      meta: { requiresAuth: true },
    },
  ],
});

// Navigation Guards
router.beforeEach(
  async (
    to: RouteLocationNormalized,
    _from: RouteLocationNormalized,
    next: NavigationGuardNext,
  ) => {
    const authStore = useAuthStore();

    // Ensure state is loaded before checking auth
    if (authStore.token === null) {
      await authStore.loadState();
    }

    const isAuthenticated = authStore.isAuthenticated;

    if (to.meta.requiresAuth && !isAuthenticated) {
      next({ name: "login" });
    } else if (to.name === "login" && isAuthenticated) {
      next({ name: "home" });
    } else {
      next();
    }
  },
);

export default router;
