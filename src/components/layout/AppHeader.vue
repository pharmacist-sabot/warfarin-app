<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useWarfarinStore } from '@/stores/warfarin';

defineProps<{
  mounted: boolean;
}>();

const warfarinStore = useWarfarinStore();
const { wasmReady } = storeToRefs(warfarinStore);
</script>

<template>
  <header
    class="text-center mb-10 transition-all duration-700 ease-out transform"
    :class="{ 'translate-y-0 opacity-100': mounted, '-translate-y-4 opacity-0': !mounted }"
  >
    <div
      class="inline-flex items-center justify-center space-x-2 bg-white px-4 py-1.5 rounded-full shadow-sm border border-gray-100 mb-6"
    >
      <span class="w-2 h-2 rounded-full" :class="wasmReady ? 'bg-green-500 animate-pulse' : 'bg-red-400'" />
      <span class="text-xs font-medium text-gray-500 tracking-wide uppercase">Dose Calculator</span>
    </div>
    <h1 class="text-4xl sm:text-5xl font-bold tracking-tight text-gray-900 mb-2 font-display">
      Warfarin
    </h1>
    <p class="text-gray-500 text-lg font-light">
      Precision Dosing Assistant
    </p>
  </header>
</template>
