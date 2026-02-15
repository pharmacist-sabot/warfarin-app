<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useWarfarinStore } from '@/stores/warfarin';

const warfarinStore = useWarfarinStore();
const { loading, wasmReady } = storeToRefs(warfarinStore);
</script>

<template>
  <div class="mt-12">
    <button
      :disabled="loading || !wasmReady"
      class="w-full py-4 rounded-2xl bg-[#0071E3] text-white font-semibold text-lg shadow-lg shadow-blue-500/30 hover:shadow-blue-500/50 hover:bg-[#0077ED] transition-all transform active:scale-[0.98] disabled:opacity-70 disabled:cursor-not-allowed flex items-center justify-center gap-2"
      @click="warfarinStore.handleCalculation()"
    >
      <span v-if="loading || !wasmReady" class="animate-spin">
        <svg
          xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"
          stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
        >
          <path d="M21 12a9 9 0 1 1-6.219-8.56" />
        </svg>
      </span>
      <span v-else>Generate Regimen</span>
    </button>
  </div>
</template>
