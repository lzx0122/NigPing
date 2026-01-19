<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import VChart from "vue-echarts";
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { MapChart, ScatterChart, EffectScatterChart } from "echarts/charts";
import {
  TitleComponent,
  TooltipComponent,
  GeoComponent,
} from "echarts/components";
import * as echarts from "echarts/core";
import worldMapData from "@/assets/world.json";

// 註冊必要的 ECharts 組件
use([
  CanvasRenderer,
  MapChart,
  ScatterChart,
  EffectScatterChart,
  TitleComponent,
  TooltipComponent,
  GeoComponent,
]);

const props = defineProps<{
  markers: { location: [number, number]; size: number }[];
}>();

const serverLocation = computed(() => {
  if (props.markers.length === 0) return null;
  return props.markers[0].location;
});

// 追蹤當前縮放等級
const currentZoom = ref(6);

// 註冊世界地圖
onMounted(() => {
  echarts.registerMap("world", worldMapData as any);
});

// ECharts 配置選項
const option = computed(() => {
  const markerData = props.markers.map((m) => ({
    name: "Server",
    value: [m.location[1], m.location[0]], // ECharts uses [longitude, latitude]
  }));

  // 計算基於縮放等級的標記大小
  const baseMarkerSize = 12; // 基礎標記大小（減小）
  const baseEffectSize = 10; // 基礎效果大小（減小）
  const markerSize = baseMarkerSize / Math.sqrt(currentZoom.value);
  const effectSize = baseEffectSize / Math.sqrt(currentZoom.value);

  return {
    backgroundColor: "transparent",
    geo: {
      map: "world",
      roam: true, // 啟用縮放和平移
      center: [120.5161, 24.0518], // 初始視角中心點：台灣彰化 [經度, 緯度]
      zoom: 6, // 初始縮放倍率（更近的視角）
      scaleLimit: {
        min: 1, // 最小縮放倍率
        max: 15, // 最大縮放倍率（增加到 15x）
      },
      itemStyle: {
        areaColor: "#3f3f46", // 提亮為 zinc-700，更容易看清楚
        borderColor: "#71717a", // 提亮為 zinc-500，邊界更明顯
        borderWidth: 1.2, // 增加邊界寬度
      },
      emphasis: {
        disabled: false, // 啟用 hover 效果
        itemStyle: {
          areaColor: "#52525b", // hover 時稍微提亮
        },
      },
      select: {
        disabled: true, // 禁用選擇
      },
      regions: [
        // 高亮台灣區域
        {
          name: "Taiwan",
          itemStyle: {
            areaColor: "#52525b", // zinc-600，讓台灣稍微亮一點
            borderColor: "#a1a1aa", // zinc-400
            borderWidth: 1.5,
          },
        },
      ],
    },
    series: [
      {
        type: "scatter",
        coordinateSystem: "geo",
        data: markerData,
        symbolSize: markerSize, // 動態標記尺寸，根據縮放調整
        symbol: "pin",
        itemStyle: {
          color: "#fbbf24", // 使用更鮮豔的 amber-400
          shadowBlur: 15,
          shadowColor: "#fbbf24",
        },
        zlevel: 2,
      },
      {
        type: "effectScatter",
        coordinateSystem: "geo",
        data: markerData,
        symbolSize: effectSize, // 動態效果尺寸，根據縮放調整
        showEffectOn: "render",
        rippleEffect: {
          brushType: "stroke",
          scale: 5, // 增大波紋範圍
          period: 3, // 加快動畫速度
        },
        itemStyle: {
          color: "#fbbf24",
          shadowBlur: 20,
          shadowColor: "#fbbf24",
        },
        zlevel: 3,
      },
    ],
    // 監聽縮放事件
    onGeoRoam: (params: any) => {
      if (params.zoom != null) {
        currentZoom.value = params.zoom;
      }
    },
  };
});
</script>

<template>
  <div
    class="w-full h-full relative overflow-hidden flex flex-col items-center justify-center bg-zinc-950"
  >
    <!-- ECharts Map -->
    <VChart class="w-full h-full" :option="option" autoresize />

    <!-- Location Info Overlay -->
    <div
      v-if="serverLocation"
      class="absolute bottom-0 left-0 right-0 flex items-center justify-between p-4 bg-gradient-to-t from-black/80 to-transparent"
    >
      <div
        class="flex items-center gap-2 bg-black/60 backdrop-blur-sm px-3 py-2 rounded border border-yellow-500/30"
      >
        <div class="w-3 h-3 rounded-full bg-yellow-500 animate-pulse"></div>
        <div class="text-xs">
          <div class="text-yellow-400 font-bold">Server Location</div>
          <div class="text-zinc-300 font-mono">
            {{ serverLocation[0].toFixed(4) }}°N,
            {{ serverLocation[1].toFixed(4) }}°E
          </div>
        </div>
      </div>

      <div
        class="text-xs text-zinc-400 font-mono bg-black/40 px-2 py-1 rounded"
      >
        Taiwan (Changhua)
      </div>
    </div>
  </div>
</template>
