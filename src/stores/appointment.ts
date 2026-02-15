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
    if (appointmentToggle.value && startDate.value && endDate.value) {
      const startDt = new Date(startDate.value);
      const endDt = new Date(endDate.value);

      if (endDt > startDt) {
        const timeDiff = endDt.getTime() - startDt.getTime();
        const daysUntilAppointment = Math.max(1, Math.round(timeDiff / (1000 * 3600 * 24)));

        const jsDay = startDt.getDay(); // JS: 0=Sun, 1=Mon...
        const startDayOfWeek = jsDay === 0 ? 6 : jsDay - 1;

        return { valid: true, daysUntilAppointment, startDayOfWeek };
      }

      // endDate <= startDate — invalid range
      return { valid: false, daysUntilAppointment: 0, startDayOfWeek: 0 };
    }

    // Toggle off or dates not yet filled — use defaults
    return { valid: true, daysUntilAppointment: 7, startDayOfWeek: 0 };
  });

  const appointmentDaysText = computed(() => {
    if (
      appointmentToggle.value
      && appointmentInfo.value.valid
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
    const yyyy = today.getFullYear();
    const mm = String(today.getMonth() + 1).padStart(2, '0');
    const dd = String(today.getDate()).padStart(2, '0');
    startDate.value = `${yyyy}-${mm}-${dd}`;
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
