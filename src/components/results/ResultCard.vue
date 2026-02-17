<script setup lang="ts">
import type { PillRenderData, RegimenOption } from '@/types';

import PillVisual from '@/components/pills/PillVisual.vue';

defineProps<{
  option: RegimenOption;
  index: number;
}>();

const DAY_NAMES = ['จ.', 'อ.', 'พ.', 'พฤ.', 'ศ.', 'ส.', 'อา.'];

const DAY_HEADER_COLORS = [
  'bg-yellow-100',
  'bg-pink-100',
  'bg-green-100',
  'bg-orange-200',
  'bg-sky-100',
  'bg-purple-100',
  'bg-red-200',
];

function getDayName(dayIndex: number): string {
  return DAY_NAMES[dayIndex] ?? '';
}

function getDayHeaderColor(dayIndex: number): string {
  return DAY_HEADER_COLORS[dayIndex] ?? '';
}

function getPillLabel(pill: PillRenderData): string {
  const countText = pill.is_half ? `x${pill.count}(ครึ่ง)` : `x${pill.count}`;
  return `${pill.mg} mg ${countText}`;
}

function expandPills(pills: PillRenderData[]): Array<{ mg: number; isHalf: boolean; key: string }> {
  const expanded: Array<{ mg: number; isHalf: boolean; key: string }> = [];
  for (const pill of pills) {
    for (let i = 0; i < pill.count; i++) {
      const pos = expanded.length;
      expanded.push({
        mg: pill.mg,
        isHalf: pill.is_half,
        key: `${pill.mg}-${pill.is_half}-${pos}`,
      });
    }
  }
  return expanded;
}
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

    <!-- Schedule Grid (Rendered from JSON Data) -->
    <div class="grid grid-cols-4 sm:grid-cols-7 gap-2 mt-4">
      <div
        v-for="day in option.weekly_schedule" :key="day.day_index"
        class="rounded-lg border text-center flex flex-col h-full overflow-hidden" :class="[
          day.is_stop_day
            ? 'bg-gray-100 border-gray-300'
            : day.is_special_day
              ? 'bg-white border-red-400 border-2'
              : 'bg-white border-gray-300',
        ]"
      >
        <!-- Day Name Header -->
        <div class="font-bold text-gray-800 py-1" :class="getDayHeaderColor(day.day_index)">
          {{ getDayName(day.day_index) }}
        </div>

        <!-- Day Content -->
        <div class="p-2 flex-grow flex flex-col">
          <!-- Dose Text (only for non-stop days) -->
          <div v-if="!day.is_stop_day" class="text-sm text-gray-800">
            ({{ day.total_dose.toFixed(1) }} mg)
          </div>

          <!-- Stop Day Content -->
          <template v-if="day.is_stop_day">
            <div class="flex-grow flex flex-col justify-center items-center my-2 min-h-[30px]">
              <div class="text-xs text-gray-500">
                ไม่มีขนาดยา
              </div>
            </div>
          </template>

          <!-- Normal / Special Day Content -->
          <template v-else>
            <div class="flex-grow flex flex-col justify-center items-center my-2 min-h-[30px]">
              <div class="flex justify-center items-center flex-wrap">
                <PillVisual
                  v-for="pill in expandPills(day.pills)" :key="pill.key" :mg="pill.mg"
                  :is-half="pill.isHalf"
                />
              </div>
            </div>

            <!-- Pill Text Labels -->
            <div>
              <div
                v-for="(pill, pillIndex) in day.pills"
                :key="`label-${pill.mg}-${pill.is_half}-${pillIndex}`" class="text-xs text-gray-600"
              >
                {{ getPillLabel(pill) }}
              </div>
            </div>
          </template>
        </div>
      </div>
    </div>

    <!-- Footer Info (structured rendering, no v-html) -->
    <div class="mt-4 pt-4 border-t border-gray-50 text-sm text-gray-600 bg-gray-50/50 rounded-lg p-3">
      <div class="font-bold">
        {{ option.total_pills_summary.header }}
      </div>
      <template v-if="option.total_pills_summary.pill_lines.length > 0">
        <div
          v-for="line in option.total_pills_summary.pill_lines" :key="`summary-pill-${line.mg}`"
          class="flex items-center gap-1 mt-1"
        >
          <PillVisual :mg="line.mg" />
          <span>{{ line.mg }}mg: {{ line.dispensed_count }} เม็ด</span>
          <span v-if="line.usage_note" class="text-gray-500">{{ line.usage_note }}</span>
        </div>
      </template>
      <div v-else class="mt-1">
        ไม่ต้องจ่ายยา
      </div>
    </div>
  </div>
</template>
