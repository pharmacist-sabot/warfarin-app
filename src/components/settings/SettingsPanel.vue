<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useAppointmentStore } from '@/stores/appointment';
import { useWarfarinStore } from '@/stores/warfarin';

import ToggleSwitch from './ToggleSwitch.vue';

const warfarinStore = useWarfarinStore();
const appointmentStore = useAppointmentStore();

const { allowHalf } = storeToRefs(warfarinStore);
const { appointmentToggle } = storeToRefs(appointmentStore);
</script>

<template>
  <div>
    <label class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-4">
      Settings
    </label>
    <div class="space-y-3">
      <!-- Allow Half Pills -->
      <label
        class="flex items-center justify-between p-3 rounded-xl bg-gray-50 hover:bg-white border border-transparent hover:border-gray-100 transition-all cursor-pointer group"
      >
        <span class="text-sm text-gray-600 group-hover:text-gray-900">
          Allow Half Pills (ครึ่งเม็ด)
        </span>
        <ToggleSwitch v-model="allowHalf" />
      </label>

      <!-- Appointment Toggle -->
      <label
        class="flex items-center justify-between p-3 rounded-xl bg-gray-50 hover:bg-white border border-transparent hover:border-gray-100 transition-all cursor-pointer group"
      >
        <span class="text-sm text-gray-600 group-hover:text-gray-900">
          Appointment Mode
        </span>
        <ToggleSwitch v-model="appointmentToggle" />
      </label>
    </div>
  </div>
</template>
