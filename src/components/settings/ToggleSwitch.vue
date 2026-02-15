<script setup lang="ts">
const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
}>();

function toggle() {
  emit('update:modelValue', !props.modelValue);
}
</script>

<template>
  <div
    class="relative inline-block w-10 mr-2 align-middle select-none transition duration-200 ease-in"
    @click.prevent.stop="toggle"
  >
    <input
      :checked="modelValue" type="checkbox"
      class="toggle-checkbox absolute block w-5 h-5 rounded-full bg-white border-4 appearance-none cursor-pointer transition-transform duration-300 ease-in-out"
      :class="modelValue ? 'translate-x-5 border-blue-500' : 'translate-x-0 border-gray-300'" @click.stop
      @change="toggle"
    >
    <div
      class="toggle-label block overflow-hidden h-5 rounded-full bg-gray-200 cursor-pointer transition-colors duration-300"
      :class="modelValue ? 'bg-blue-200' : ''"
    />
  </div>
</template>
