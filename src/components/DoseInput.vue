<script setup lang="ts">
const props = defineProps<{
  modelValue: number | null;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: number | null];
  'calculate': [];
}>();

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  const val = target.value === '' ? null : Number.parseFloat(target.value);
  emit('update:modelValue', val);
}

function adjustDose(percent: number) {
  let current = props.modelValue ?? 0;
  if (Number.isNaN(current))
    current = 0;

  if (percent === 0) {
    // "Same" — keep current value as-is
    return;
  }

  const next = current * (1 + percent / 100);
  emit('update:modelValue', Math.round(next * 2) / 2); // Round to nearest 0.5
}
</script>

<template>
  <div class="mb-10 text-center">
    <label class="block text-sm font-semibold text-gray-400 uppercase tracking-wider mb-4">
      Target Weekly Dose (mg)
    </label>
    <div class="relative inline-block group">
      <input
        type="number" :value="modelValue"
        class="input-reset text-6xl sm:text-7xl font-light text-gray-900 placeholder-gray-200 w-full max-w-[300px] border-b-2 border-transparent focus:border-blue-500 transition-colors duration-300 pb-2"
        placeholder="0" step="0.5" @input="handleInput" @keydown.enter="$emit('calculate')"
      >
      <span class="text-xl text-gray-400 absolute top-4 -right-8 font-light">mg</span>
    </div>

    <!-- Quick Adjustments -->
    <div class="flex justify-center gap-3 mt-6">
      <button
        class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-red-50 hover:text-red-600 transition-colors scale-tap"
        @click="adjustDose(-10)"
      >
        -10%
      </button>
      <button
        class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-gray-100 transition-colors scale-tap"
        @click="adjustDose(0)"
      >
        Same
      </button>
      <button
        class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-green-50 hover:text-green-600 transition-colors scale-tap"
        @click="adjustDose(10)"
      >
        +10%
      </button>
    </div>
  </div>
</template>
