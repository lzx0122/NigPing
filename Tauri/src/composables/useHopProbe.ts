import { onUnmounted, ref } from "vue";
import {
  getHopStats,
  startHopProbe as tauriStartProbe,
  stopHopProbe as tauriStopProbe,
  type HopStatPayload,
} from "@/lib/tauriCommands";

export function useHopProbe() {
  const isProbing = ref(false);
  const isLoading = ref(false);
  const hopStats = ref<HopStatPayload[]>([]);
  const statusMessage = ref("");
  const currentTarget = ref("");

  let pollInterval: number | null = null;
  const POLLING_INTERVAL_MS = 1000;

  function clearPollingTimer() {
    if (pollInterval !== null) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  }

  async function fetchStats() {
    if (!isProbing.value) return;
    try {
      const stats = await getHopStats();
      hopStats.value = stats;
    } catch (error) {
      console.error("Failed to fetch hop stats:", error);
    }
  }

  async function startProbe(target: string) {
    if (!target.trim()) return;

    isLoading.value = true;
    statusMessage.value = `Starting probe via ICMP...`;
    hopStats.value = [];
    currentTarget.value = target;

    try {
      // clean up any running probes without asserting failure
      await tauriStopProbe().catch(() => {});

      await tauriStartProbe({ target: target.trim() });
      isProbing.value = true;
      statusMessage.value = `Probing ${target} via ICMP...`;

      clearPollingTimer();
      pollInterval = window.setInterval(fetchStats, POLLING_INTERVAL_MS);
    } catch (error) {
      statusMessage.value = `Failed: ${String(error)}`;
      isProbing.value = false;
      currentTarget.value = "";
    } finally {
      isLoading.value = false;
    }
  }

  async function stopProbe() {
    isLoading.value = true;
    
    try {
      if (isProbing.value) {
        await tauriStopProbe().catch(() => {});
      }
      isProbing.value = false;
      statusMessage.value = "Stopped";
      currentTarget.value = "";
      clearPollingTimer();
    } catch (error) {
      statusMessage.value = `Stop failed: ${String(error)}`;
    } finally {
      isLoading.value = false;
    }
  }

  onUnmounted(() => {
    clearPollingTimer();
    if (isProbing.value) {
      void tauriStopProbe().catch(() => {});
    }
  });

  return {
    isProbing,
    isLoading,
    hopStats,
    statusMessage,
    currentTarget,
    startProbe,
    stopProbe,
  };
}
