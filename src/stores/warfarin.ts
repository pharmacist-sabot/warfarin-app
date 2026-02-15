import { defineStore } from 'pinia';
import { nextTick, ref } from 'vue';

import type { AvailablePills, RegimenOption } from '@/types';

import { DEFAULT_AVAILABLE_PILLS } from '@/constants/pills';

import init, { generate_suggestions_rust } from '../../warfarin_logic/pkg/warfarin_logic.js';
import { useAppointmentStore } from './appointment';

export const useWarfarinStore = defineStore('warfarin', () => {
  // --- State ---
  const weeklyDose = ref<number | null>(null);
  const allowHalf = ref(true);
  const availablePills = ref<AvailablePills>({ ...DEFAULT_AVAILABLE_PILLS });
  const specialDayPattern = ref('fri-sun');
  const results = ref<RegimenOption[]>([]);
  const loading = ref(false);
  const errorMsg = ref('');
  const wasmReady = ref(false);

  // --- Actions ---
  async function initWasm() {
    try {
      await init();
      wasmReady.value = true;
      console.warn('WASM module initialized!');
    }
    catch (e) {
      console.error('Failed to initialize WASM module', e);
      errorMsg.value = 'ไม่สามารถโหลดระบบคำนวณได้ กรุณารีเฟรชหน้าเว็บ';
    }
  }

  async function handleCalculation() {
    if (weeklyDose.value === null || weeklyDose.value < 0) {
      errorMsg.value = 'กรุณากรอกขนาดยาเป้าหมายให้ถูกต้อง';
      results.value = [];
      return;
    }

    const selectedPills = Object.keys(availablePills.value)
      .filter(key => availablePills.value[Number(key)]);

    if (selectedPills.length === 0) {
      errorMsg.value = 'กรุณาเลือกขนาดยาอย่างน้อย 1 ขนาด';
      results.value = [];
      return;
    }

    loading.value = true;
    errorMsg.value = '';
    results.value = [];

    // Simulate small delay for UI feedback feeling
    await new Promise(r => setTimeout(r, 400));

    try {
      const appointmentStore = useAppointmentStore();
      const appointmentInfo = appointmentStore.appointmentInfo;

      const input = {
        weekly_dose: Number.parseFloat(String(weeklyDose.value)),
        allow_half: allowHalf.value,
        available_pills: selectedPills.map(Number),
        special_day_pattern: specialDayPattern.value,
        days_until_appointment: appointmentInfo.daysUntilAppointment,
        start_day_of_week: appointmentInfo.startDayOfWeek,
      };

      const rustResults = await generate_suggestions_rust(input);
      results.value = rustResults;

      if (rustResults.length === 0) {
        errorMsg.value = 'ไม่พบตัวเลือกที่เหมาะสมสำหรับเงื่อนไขนี้ (ลองอนุญาตให้ใช้ครึ่งเม็ด)';
      }
      else {
        nextTick(() => {
          const resultsEl = document.getElementById('results-section');
          if (resultsEl)
            resultsEl.scrollIntoView({ behavior: 'smooth', block: 'start' });
        });
      }
    }
    catch (e) {
      console.error('Error calling Rust WASM function:', e);
      errorMsg.value = `เกิดข้อผิดพลาดในการคำนวณ: ${e}`;
    }
    finally {
      loading.value = false;
    }
  }

  function clearResults() {
    results.value = [];
    errorMsg.value = '';
  }

  return {
    // State
    weeklyDose,
    allowHalf,
    availablePills,
    specialDayPattern,
    results,
    loading,
    errorMsg,
    wasmReady,

    // Actions
    initWasm,
    handleCalculation,
    clearResults,
  };
});
