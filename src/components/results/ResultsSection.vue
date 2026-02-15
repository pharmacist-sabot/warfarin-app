<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useWarfarinStore } from '@/stores/warfarin';

import ResultCard from './ResultCard.vue';

const warfarinStore = useWarfarinStore();
const { results } = storeToRefs(warfarinStore);
</script>

<template>
  <div v-if="results.length > 0" id="results-section" class="mb-20 scroll-mt-6">
    <div class="flex items-center justify-between mb-6 px-2">
      <h2 class="text-xl font-semibold text-gray-900">
        Suggested Regimens
      </h2>
      <span class="text-xs font-medium bg-gray-200 text-gray-600 px-2 py-1 rounded-md">
        {{ results.length }} Options
      </span>
    </div>

    <transition-group name="list" tag="div" class="space-y-4">
      <ResultCard v-for="(option, index) in results" :key="index" :option="option" :index="index" />
    </transition-group>
  </div>
</template>
