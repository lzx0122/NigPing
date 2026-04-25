<script setup lang="ts">
import { ref, computed } from 'vue';
import { useHopProbe } from '@/composables/useHopProbe';
import Button from '@/components/ui/button/Button.vue';
import Input from '@/components/ui/input/Input.vue';

const {
  isProbing,
  isLoading,
  hopStats,
  statusMessage,
  startProbe,
  stopProbe,
} = useHopProbe();

const targetInput = ref('');

function toggleProbe() {
  if (isProbing.value) {
    stopProbe();
  } else {
    if (targetInput.value) {
      startProbe(targetInput.value);
    }
  }
}

const getPingColor = (ms: number) => {
  if (ms === 0) return 'text-zinc-600'; // Initial or timeout
  if (ms < 50) return 'text-emerald-400 drop-shadow-[0_0_2px_rgba(52,211,153,0.8)]';
  if (ms < 150) return 'text-amber-400 drop-shadow-[0_0_2px_rgba(251,191,36,0.8)]';
  return 'text-rose-400 drop-shadow-[0_0_2px_rgba(2fb,113,133,0.8)]';
};

const getStatusColor = computed(() => {
  if (isLoading.value) return 'text-amber-400 animate-pulse';
  if (isProbing.value) return 'text-emerald-400 animate-pulse';
  return 'text-zinc-500';
});

const isCopied = ref(false);

const exportReport = async () => {
  if (!hopStats.value.length) return;

  const now = new Date().toLocaleString();
  let text = `PingPal Tactical Network Probe Report\n`;
  text += `Target: ${targetInput.value} (ICMP)\n`;
  text += `Generated: ${now}\n\n`;

  // Header
  text += `Hop | IP Address      | Loss% | Sent/Recv | Last(ms) | Avg(ms) | Best(ms) | Worst(ms)\n`;
  text += `-------------------------------------------------------------------------------------\n`;

  for (const hop of hopStats.value) {
    const hopNum = hop.hop.toString().padStart(3, ' ');
    const ip = (hop.ip || 'Request Timed Out.').padEnd(15, ' ');
    const loss = `${Math.round(hop.loss_pct)}%`.padStart(5, ' ');
    const sentRecv = `${hop.sent}/${hop.recv}`.padStart(9, ' ');
    const last = hop.last_ms > 0 ? hop.last_ms.toFixed(1).padStart(8, ' ') : '     -.-';
    const avg = hop.avg_ms > 0 ? hop.avg_ms.toFixed(1).padStart(7, ' ') : '    -.-';
    const best = hop.best_ms > 0 ? hop.best_ms.toFixed(1).padStart(8, ' ') : '     -.-';
    const worst = hop.worst_ms > 0 ? hop.worst_ms.toFixed(1).padStart(9, ' ') : '      -.-';

    text += `${hopNum} | ${ip} | ${loss} | ${sentRecv} | ${last} | ${avg} | ${best} | ${worst}\n`;
  }

  try {
    await navigator.clipboard.writeText(text);
    isCopied.value = true;
    setTimeout(() => {
      isCopied.value = false;
    }, 2000);
  } catch (err) {
    console.error('Failed to copy text: ', err);
  }
};
</script>

<template>
  <div class="probe-container relative flex flex-col gap-4 p-5 rounded-xl bg-zinc-950 border border-zinc-800/80 shadow-2xl overflow-hidden font-sans">
    <!-- Decorative Grid Background -->
    <div class="absolute inset-0 pointer-events-none grid-pattern opacity-20"></div>
    <div class="absolute top-0 left-0 w-full h-px bg-gradient-to-r from-transparent via-emerald-500/20 to-transparent"></div>

    <header class="relative flex flex-col sm:flex-row sm:items-end justify-between gap-4 border-b border-zinc-800/60 pb-4">
      <div>
        <div class="flex items-center gap-2 mb-1">
          <div class="w-2 h-2 rounded-full" :class="isProbing ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.8)]' : 'bg-zinc-700'"></div>
          <h2 class="text-xs font-bold uppercase tracking-[0.2em] text-zinc-400">Tactical Network Probe</h2>
        </div>
        <p class="text-xs text-zinc-500 font-mono tracking-tight pl-4">
          <span :class="getStatusColor">[{{ statusMessage || (isProbing ? 'ACTIVE' : 'STANDBY') }}]</span>
        </p>
      </div>

      <div class="flex flex-col sm:flex-row items-center justify-end gap-2 w-full sm:w-auto z-10">
        <div class="flex items-center gap-2 w-full sm:w-auto relative">
          <Input
            v-model="targetInput"
            placeholder="TARGET IP / HOST"
            class="h-9 w-full sm:w-48 bg-zinc-900/80 border border-zinc-700/60 rounded text-xs font-mono text-emerald-400 placeholder:text-zinc-600 focus-visible:ring-1 focus-visible:ring-emerald-500/30 pl-3 pr-2 transition-all shadow-inner"
            :disabled="isProbing || isLoading"
            @keydown.enter="toggleProbe"
          />
          <Button
            @click="exportReport"
            :disabled="hopStats.length === 0"
            variant="outline"
            class="relative h-9 px-4 ml-1 rounded text-xs font-bold uppercase tracking-wider transition-all duration-300 border-zinc-700/60 hover:border-emerald-500/50 hover:text-emerald-400 text-zinc-400 bg-zinc-900/60 shadow-lg group"
          >
            <span class="relative z-10">{{ isCopied ? 'Copied' : 'Export' }}</span>
          </Button>
          <Button
            @click="toggleProbe"
            :disabled="isLoading || (!targetInput && !isProbing)"
            class="relative h-9 px-6 ml-1 rounded text-xs font-bold uppercase tracking-widest transition-all duration-300 overflow-hidden group shadow-lg"
            :class="isProbing 
              ? 'text-rose-400 border border-rose-900/50 bg-rose-950/20 hover:bg-rose-900/40' 
              : 'text-zinc-300 border border-zinc-700 bg-zinc-800/50 hover:text-white hover:border-zinc-500 hover:bg-zinc-700'"
          >
            <span class="relative z-10">{{ isProbing ? 'Terminate' : 'Deploy' }}</span>
          </Button>
        </div>
      </div>
    </header>

    <div class="relative rounded-lg border border-zinc-800/60 bg-zinc-900/20 h-[380px] overflow-hidden backdrop-blur-sm z-10 shadow-inner">
      <div class="absolute inset-0 overflow-y-auto no-scrollbar">
        <table class="w-full text-left border-collapse">
          <thead class="sticky top-0 bg-zinc-950/90 backdrop-blur-md z-20 shadow-[0_4px_20px_rgba(0,0,0,0.5)]">
            <tr>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800">Hop</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800">Node IP</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Loss</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Sent/Recv</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Last</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Avg</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Best</th>
              <th class="px-4 py-3 text-[10px] font-bold uppercase tracking-widest text-zinc-500 border-b border-zinc-800 text-right">Worst</th>
            </tr>
          </thead>
          <tbody class="font-mono text-[11px]">
            <tr 
              v-for="(stat, index) in hopStats" 
              :key="stat.hop"
              class="border-b border-zinc-800/30 hover:bg-zinc-800/40 transition-colors group"
              :style="{ animationDelay: `${index * 50}ms` }"
            >
              <td class="px-4 py-2.5 text-zinc-400">
                <div class="flex items-center gap-2">
                  <span class="w-4 inline-block text-right">{{ stat.hop }}</span>
                  <div class="h-[1px] flex-1 bg-zinc-800 group-hover:bg-zinc-700 transition-colors"></div>
                </div>
              </td>
              <td class="px-4 py-2.5 text-zinc-300">
                {{ stat.ip }}
              </td>
              <td class="px-4 py-2.5 text-right font-bold w-20" :class="stat.loss_pct > 0 ? 'text-rose-400 drop-shadow-[0_0_4px_rgba(2fb,113,133,0.8)]' : 'text-zinc-600'">
                {{ stat.loss_pct.toFixed(0) }}%
              </td>
              <td class="px-4 py-2.5 text-right text-zinc-500">
                <span class="text-zinc-400">{{ stat.sent }}</span>
                <span class="opacity-50 mx-1">/</span>
                <span :class="stat.recv < stat.sent ? 'text-rose-400' : 'text-emerald-500'">{{ stat.recv }}</span>
              </td>
              <td class="px-4 py-2.5 text-right w-20" :class="getPingColor(stat.last_ms)">
                {{ stat.last_ms ? stat.last_ms.toFixed(1) : '-' }}
              </td>
              <td class="px-4 py-2.5 text-right w-20 text-zinc-400">
                {{ stat.avg_ms ? stat.avg_ms.toFixed(1) : '-' }}
              </td>
              <td class="px-4 py-2.5 text-right w-20 text-zinc-500">
                {{ stat.best_ms ? stat.best_ms.toFixed(1) : '-' }}
              </td>
              <td class="px-4 py-2.5 text-right w-20 text-zinc-500">
                {{ stat.worst_ms ? stat.worst_ms.toFixed(1) : '-' }}
              </td>
            </tr>
            
            <tr v-if="hopStats.length === 0">
               <td colspan="8" class="px-4 py-16 text-center">
                 <div class="flex flex-col items-center justify-center space-y-3 opacity-30">
                    <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-zinc-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                      <circle cx="12" cy="12" r="10"/>
                      <path d="M12 16v-4"/>
                      <path d="M12 8h.01"/>
                    </svg>
                    <span class="text-xs uppercase tracking-[0.2em] font-sans font-medium text-zinc-300">Awaiting Target Lock</span>
                 </div>
               </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style scoped>
.probe-container {
  background-image: radial-gradient(circle at top right, rgba(16, 185, 129, 0.03), transparent 400px),
                    radial-gradient(circle at bottom left, rgba(16, 185, 129, 0.015), transparent 300px);
}

.grid-pattern {
  background-size: 24px 24px;
  background-image: linear-gradient(to right, rgba(255, 255, 255, 0.02) 1px, transparent 1px),
                    linear-gradient(to bottom, rgba(255, 255, 255, 0.02) 1px, transparent 1px);
  mask-image: linear-gradient(to bottom, black 40%, transparent 100%);
}

.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}

/* Subtle entrance animation for table rows */
tbody tr {
  animation: fadeIn 0.4s ease-out forwards;
  opacity: 0;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(4px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
