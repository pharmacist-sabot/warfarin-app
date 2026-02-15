<script setup lang="ts">
defineProps<{
  option: {
    description: string;
    weekly_dose_actual: number;
    weekly_schedule_html: string;
    total_pills_message: string;
  };
  index: number;
}>();
</script>

<template>
  <div
    class="glass-card-white bg-white rounded-2xl p-6 shadow-sm border border-gray-100 hover:shadow-md transition-shadow"
  >
    <!-- Header of Result Card -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between mb-4 pb-4 border-b border-gray-50">
      <div>
        <div class="text-xs font-bold text-blue-600 uppercase tracking-wide mb-1">
          Option {{ index + 1 }}
        </div>
        <div class="text-gray-900 font-medium text-lg">
          {{ option.description }}
        </div>
      </div>
      <div class="mt-2 sm:mt-0 text-right">
        <div class="text-2xl font-bold text-gray-900">
          {{ option.weekly_dose_actual.toFixed(1) }}
          <span class="text-sm font-normal text-gray-400">mg/wk</span>
        </div>
      </div>
    </div>

    <!-- Schedule Grid (Rendered from Rust HTML) -->
    <div class="rust-html-content" v-html="option.weekly_schedule_html" />

    <!-- Footer Info -->
    <div class="mt-4 pt-4 border-t border-gray-50 text-sm text-gray-600 bg-gray-50/50 rounded-lg p-3">
      <div v-html="option.total_pills_message" />
    </div>
  </div>
</template>
