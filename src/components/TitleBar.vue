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
    class="h-10 bg-black flex items-center justify-between select-none border-b border-zinc-900"
    data-tauri-drag-region
  >
    <!-- Left: App branding -->
    <div class="flex items-center gap-2 pl-4" data-tauri-drag-region>
      <div class="w-4 h-4 rounded-sm bg-white flex items-center justify-center">
        <span class="text-[10px] font-black text-black leading-none">N</span>
      </div>
      <span class="text-xs font-bold text-zinc-400 tracking-wider uppercase"
        >NigPing</span
      >
    </div>

    <!-- Right: Window controls -->
    <div class="flex items-center h-full">
      <button
        @click="minimize"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-zinc-900 transition-colors"
      >
        <Minus class="w-4 h-4" />
      </button>
      <button
        @click="toggleMaximize"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-zinc-900 transition-colors"
      >
        <Square class="w-3.5 h-3.5" />
      </button>
      <button
        @click="close"
        class="h-full w-12 flex items-center justify-center text-zinc-500 hover:text-white hover:bg-red-600 transition-colors"
      >
        <X class="w-4 h-4" />
      </button>
    </div>
  </div>
</template>
