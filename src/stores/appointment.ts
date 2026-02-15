import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import type { AppointmentInfo } from '@/types';

export const useAppointmentStore = defineStore('appointment', () => {
  // --- State ---
  const appointmentToggle = ref(false);
  const startDate = ref('');
  const endDate = ref('');

  // --- Getters ---
  const appointmentInfo = computed<AppointmentInfo>(() => {
    let daysUntilAppointment = 7; // Default
    let startDayOfWeek = 0; // 0=Mon, ..., 6=Sun

    if (appointmentToggle.value && startDate.value && endDate.value) {
      const startDt = new Date(startDate.value);
      const endDt = new Date(endDate.value);
      if (endDt > startDt) {
        const timeDiff = endDt.getTime() - startDt.getTime();
        daysUntilAppointment = Math.max(1, Math.round(timeDiff / (1000 * 3600 * 24)));

        const jsDay = startDt.getDay(); // JS: 0=Sun, 1=Mon...
        startDayOfWeek = jsDay === 0 ? 6 : jsDay - 1;
      }
    }
    return { daysUntilAppointment, startDayOfWeek };
  });

  const appointmentDaysText = computed(() => {
    if (
      appointmentToggle.value
      && appointmentInfo.value.daysUntilAppointment > 0
      && startDate.value
      && endDate.value
    ) {
      return `คำนวณสำหรับ ${appointmentInfo.value.daysUntilAppointment} วัน`;
    }
    return '';
  });

  // --- Actions ---
  function initializeStartDate() {
    const today = new Date();
    startDate.value = today.toISOString().split('T')[0] ?? '';
  }

  function setStartDate(date: string) {
    startDate.value = date;
  }

  function setEndDate(date: string) {
    endDate.value = date;
  }

  function toggleAppointment() {
    appointmentToggle.value = !appointmentToggle.value;
  }

  return {
    // State
    appointmentToggle,
    startDate,
    endDate,
    // Getters
    appointmentInfo,
    appointmentDaysText,
    // Actions
    initializeStartDate,
    setStartDate,
    setEndDate,
    toggleAppointment,
  };
});
