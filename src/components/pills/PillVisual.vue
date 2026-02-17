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
    <span class="text-[10px] font-bold text-white/90">
      {{ isHalf ? '½' : mg }}
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
  @apply bg-gray-300 border border-gray-200;
}

.pill-2 {
  @apply bg-orange-300;
}

.pill-3 {
  @apply bg-sky-400;
}

.pill-5 {
  @apply bg-pink-400;
}
</style>
