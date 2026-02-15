<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useWarfarinStore } from '@/stores/warfarin';

const warfarinStore = useWarfarinStore();
const { errorMsg } = storeToRefs(warfarinStore);
</script>

<template>
  <transition name="fade">
    <div
      v-if="errorMsg"
      class="text-center p-6 bg-red-50 text-red-600 rounded-2xl mb-8 border border-red-100 shadow-sm"
    >
      {{ errorMsg }}
    </div>
  </transition>
</template>
