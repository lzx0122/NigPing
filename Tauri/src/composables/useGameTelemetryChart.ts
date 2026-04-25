import { computed, ref, watch, type ComputedRef, type Ref } from "vue";
import type { DetectedServerPayload } from "@/lib/tauriCommands";

export type TelemetryPoint = {
  send: number;
  recv: number;
  ping: number | null;
};

const DEFAULT_MAX_HISTORY = 60;

function getHistoryStorageKey(gameId: string): string {
  return `pingpal:telemetry:${gameId}`;
}

function normalizeTelemetryPoint(value: unknown): TelemetryPoint | null {
  if (!value || typeof value !== "object") return null;
  const point = value as Record<string, unknown>;
  if (typeof point.send !== "number" || typeof point.recv !== "number") {
    return null;
  }
  const ping =
    typeof point.ping === "number" || point.ping === null ? point.ping : null;
  return { send: point.send, recv: point.recv, ping };
}

function loadPersistedHistory(
  gameId: string,
  maxHistory: number,
): TelemetryPoint[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = window.sessionStorage.getItem(getHistoryStorageKey(gameId));
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    if (!Array.isArray(parsed)) return [];
    return parsed
      .map((entry) => normalizeTelemetryPoint(entry))
      .filter((entry): entry is TelemetryPoint => entry !== null)
      .slice(-maxHistory);
  } catch {
    return [];
  }
}

function persistHistory(
  gameId: string,
  points: TelemetryPoint[],
  maxHistory: number,
) {
  if (typeof window === "undefined") return;
  try {
    window.sessionStorage.setItem(
      getHistoryStorageKey(gameId),
      JSON.stringify(points.slice(-maxHistory)),
    );
  } catch {
  }
}

export type UseGameTelemetryChartOptions = {
  getGameId: () => string;
  getPrimary: () => DetectedServerPayload | null | undefined;
  getIsTrafficMonitoring: () => boolean | undefined;
  maxHistory?: number;
};

export function useGameTelemetryChart(options: UseGameTelemetryChartOptions): {
  history: Ref<TelemetryPoint[]>;
  pingDisplayText: ComputedRef<string>;
  sendPathLine: ComputedRef<string>;
  sendPathArea: ComputedRef<string>;
  recvPathLine: ComputedRef<string>;
  recvPathArea: ComputedRef<string>;
  pingPathLine: ComputedRef<string>;
} {
  const maxHistory = options.maxHistory ?? DEFAULT_MAX_HISTORY;

  const history = ref<TelemetryPoint[]>(
    loadPersistedHistory(options.getGameId(), maxHistory),
  );

  const currentTelemetrySample = computed(() => {
    const data = options.getPrimary();
    if (!data) return null;
    return {
      send: data.send_rate,
      recv: data.recv_rate,
      ping: data.ping_ms,
    };
  });

  const currentEndpointKey = computed(() => {
    const data = options.getPrimary();
    if (!data) return null;
    return `${data.protocol}:${data.ip}:${data.port}`;
  });

  const pingDisplayText = computed(() => {
    const ping = options.getPrimary()?.ping_ms;
    if (ping === null || ping === undefined) return "--";
    return `${ping} ms`;
  });

  watch(
    currentTelemetrySample,
    (sample) => {
      if (sample) {
        history.value.push({
          send: sample.send,
          recv: sample.recv,
          ping: sample.ping,
        });
        if (history.value.length > maxHistory) {
          history.value.shift();
        }
        persistHistory(options.getGameId(), history.value, maxHistory);
      }
    },
  );

  watch(currentEndpointKey, (next, prev) => {
    if (next !== prev) {
      history.value = [];
      persistHistory(options.getGameId(), history.value, maxHistory);
    }
  });

  watch(
    () => options.getGameId(),
    (nextGameId) => {
      history.value = loadPersistedHistory(nextGameId, maxHistory);
    },
  );

  watch(
    () => options.getIsTrafficMonitoring(),
    (on) => {
      if (!on) {
        history.value = [];
        persistHistory(options.getGameId(), history.value, maxHistory);
      }
    },
  );

  const maxChartValue = computed(() => {
    if (history.value.length < 2) return 1024;
    return Math.max(
      ...history.value.map((d) => Math.max(d.send, d.recv)),
      1024,
    );
  });

  const maxPingChartValue = computed(() => {
    const pings = history.value
      .map((d) => d.ping)
      .filter((v): v is number => typeof v === "number");
    if (pings.length < 2) return 100;
    return Math.max(...pings, 100);
  });

  function createPath(key: "send" | "recv", isArea: boolean) {
    if (history.value.length < 2) return "";
    const width = 100;
    const height = 100;
    const maxVal = maxChartValue.value;
    const points = history.value.map((d, i) => {
      const x = (i / (history.value.length - 1)) * width;
      const y = height - (d[key] / maxVal) * height;
      return `${x.toFixed(1)},${y.toFixed(1)}`;
    });
    if (isArea) {
      const firstX = points[0].split(",")[0];
      const lastX = points[points.length - 1].split(",")[0];
      return `M ${firstX},${height} L ${points.join(" L ")} L ${lastX},${height} Z`;
    }
    return `M ${points.join(" L ")}`;
  }

  function createPingLinePath() {
    if (history.value.length < 2) return "";
    const segments: string[] = [];
    let currentSegment: string[] = [];
    const width = 100;
    const height = 100;
    const maxVal = maxPingChartValue.value;

    history.value.forEach((d, i) => {
      if (d.ping === null || d.ping === undefined) {
        if (currentSegment.length >= 2) {
          segments.push(`M ${currentSegment.join(" L ")}`);
        }
        currentSegment = [];
        return;
      }
      const x = (i / (history.value.length - 1)) * width;
      const y = height - (d.ping / maxVal) * height;
      currentSegment.push(`${x.toFixed(1)},${y.toFixed(1)}`);
    });

    if (currentSegment.length >= 2) {
      segments.push(`M ${currentSegment.join(" L ")}`);
    }

    if (segments.length === 0) return "";
    return segments.join(" ");
  }

  const sendPathLine = computed(() => createPath("send", false));
  const sendPathArea = computed(() => createPath("send", true));
  const recvPathLine = computed(() => createPath("recv", false));
  const recvPathArea = computed(() => createPath("recv", true));
  const pingPathLine = computed(() => createPingLinePath());

  return {
    history,
    pingDisplayText,
    sendPathLine,
    sendPathArea,
    recvPathLine,
    recvPathArea,
    pingPathLine,
  };
}
