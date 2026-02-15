<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { PILL_TYPES } from '@/constants/pills';
import { useWarfarinStore } from '@/stores/warfarin';

const warfarinStore = useWarfarinStore();
const { availablePills } = storeToRefs(warfarinStore);

function togglePill(mg: number) {
  availablePills.value = { ...availablePills.value, [mg]: !availablePills.value[mg] };
}
</script>

<template>
  <div>
    <label class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-4">
      Available Pills
    </label>
    <div class="flex flex-wrap gap-3">
      <button
        v-for="pill in PILL_TYPES" :key="pill.mg"
        class="relative group flex items-center gap-3 pl-2 pr-4 py-2 rounded-full border transition-all duration-300 scale-tap"
        :class="availablePills[pill.mg] ? 'bg-white border-gray-200 shadow-md ring-1 ring-black/5' : 'bg-gray-50 border-transparent opacity-60 grayscale'"
        @click="togglePill(pill.mg)"
      >
        <!-- Pill Visual -->
        <div
          class="w-8 h-8 rounded-full shadow-inner flex items-center justify-center text-[10px] font-bold text-white/90"
          :class="pill.colorClass"
        >
          {{ pill.mg }}
        </div>
        <span class="text-sm font-medium text-gray-700 group-hover:text-gray-900">{{ pill.mg }} mg</span>

        <!-- Checkmark Icon -->
        <div
          v-if="availablePills[pill.mg]"
          class="absolute top-0 right-0 -mt-1 -mr-1 bg-green-500 text-white rounded-full p-0.5 shadow-sm"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none"
            stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"
          >
            <polyline points="20 6 9 17 4 12" />
          </svg>
        </div>
      </button>
    </div>
  </div>
</template>
