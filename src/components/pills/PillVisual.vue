<script setup lang="ts">
defineProps<{
  mg: number;
  isHalf?: boolean;
}>();
</script>

<template>
  <span
    class="pill inline-flex items-center justify-center rounded-full shadow-inner" :class="[
      `pill-${mg}`,
      isHalf ? 'pill-half-left' : '',
    ]"
  >
    <span v-if="!isHalf" class="text-[10px] font-bold text-white/90">
      {{ mg }}
    </span>
  </span>
</template>

<style scoped>
.pill {
  width: 28px;
  height: 28px;
  margin: 2px;
  vertical-align: middle;
}

.pill-half-left {
  clip-path: polygon(0 0, 50% 0, 50% 100%, 0% 100%);
}

/* Color classes mapping based on design system */
.pill-1 {
  background-color: #d1d5db;
  border: 1px solid #e5e7eb;
}

.pill-2 {
  background-color: #fdba74;
}

.pill-3 {
  background-color: #38bdf8;
}

.pill-5 {
  background-color: #f472b6;
}
</style>
