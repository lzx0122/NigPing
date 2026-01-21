<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-vue-next";

const appWindow = getCurrentWindow();

const minimize = () => appWindow.minimize();
const toggleMaximize = () => appWindow.toggleMaximize();
const close = () => appWindow.close();
</script>

<template>
  <div
    class="h-11 bg-white/5 backdrop-blur-xl border-b border-white/10 flex items-center justify-between select-none shadow-sm"
    data-tauri-drag-region
  >
    <!-- Left: App branding -->
    <div class="flex items-center gap-2.5 pl-4" data-tauri-drag-region>
      <div
        class="w-5 h-5 rounded-md bg-gradient-to-br from-amber-400 via-orange-500 to-red-500 flex items-center justify-center shadow-lg shadow-orange-500/20"
      >
        <span class="text-[10px] font-black text-white tracking-tight">N</span>
      </div>
      <span class="text-sm font-semibold text-zinc-300 tracking-wide"
        >NigPing</span
      >
    </div>

    <!-- Right: Window controls (macOS/Spotify style positioning, Windows icons) -->
    <div class="flex items-center h-full">
      <button
        @click="minimize"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-zinc-200 hover:bg-zinc-800/80 transition-all duration-150"
      >
        <Minus class="w-4 h-4" :stroke-width="1.5" />
      </button>
      <button
        @click="toggleMaximize"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-zinc-200 hover:bg-zinc-800/80 transition-all duration-150"
      >
        <Square class="w-3.5 h-3.5" :stroke-width="1.5" />
      </button>
      <button
        @click="close"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-red-600 transition-all duration-150"
      >
        <X class="w-4 h-4" :stroke-width="1.5" />
      </button>
    </div>
  </div>
</template>
