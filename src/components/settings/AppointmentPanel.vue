<script setup lang="ts">
import { storeToRefs } from 'pinia';

import { useAppointmentStore } from '@/stores/appointment';

const appointmentStore = useAppointmentStore();
const { startDate, endDate, appointmentDaysText } = storeToRefs(appointmentStore);
</script>

<template>
  <div class="mt-6 bg-blue-50/50 rounded-2xl p-5 border border-blue-100">
    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
      <div>
        <label class="block text-xs font-semibold text-blue-800 mb-1">
          Start Date (วันที่มา)
        </label>
        <input
          v-model="startDate" type="date"
          class="w-full bg-white border border-blue-200 text-gray-700 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 outline-none"
        >
      </div>
      <div>
        <label class="block text-xs font-semibold text-blue-800 mb-1">
          Next Appointment (วันนัด)
        </label>
        <input
          v-model="endDate" type="date"
          class="w-full bg-white border border-blue-200 text-gray-700 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 outline-none"
        >
      </div>
    </div>
    <div v-if="appointmentDaysText" class="mt-3 text-center text-sm font-medium text-blue-600">
      {{ appointmentDaysText }}
    </div>
  </div>
</template>
