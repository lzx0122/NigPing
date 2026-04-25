import { defineStore } from "pinia";
import { ref } from "vue";

export const useTrafficMonitorStore = defineStore("trafficMonitor", () => {
  const ownerSeq = ref(0);
  const monitorOwner = ref<string | null>(null);
  const startInFlight = ref(false);
  const monitoringActive = ref(false);
  const activeProcessName = ref<string | null>(null);

  function nextOwnerId() {
    ownerSeq.value += 1;
    return `traffic-monitor-${ownerSeq.value}`;
  }

  function setStartInFlight(value: boolean) {
    startInFlight.value = value;
  }

  function claimOwner(ownerId: string) {
    monitorOwner.value = ownerId;
  }

  function releaseOwner() {
    monitorOwner.value = null;
  }

  function markMonitoringStarted(processName: string) {
    monitoringActive.value = true;
    activeProcessName.value = processName;
  }

  function markMonitoringStopped() {
    monitoringActive.value = false;
    activeProcessName.value = null;
  }

  return {
    monitorOwner,
    startInFlight,
    monitoringActive,
    activeProcessName,
    nextOwnerId,
    setStartInFlight,
    claimOwner,
    releaseOwner,
    markMonitoringStarted,
    markMonitoringStopped,
  };
});
