<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { onMounted, ref, watch } from 'vue';

import CalculateButton from '@/components/dose/CalculateButton.vue';
import DoseInput from '@/components/dose/DoseInput.vue';
import AppFooter from '@/components/layout/AppFooter.vue';
import AppHeader from '@/components/layout/AppHeader.vue';
import BackgroundDecor from '@/components/layout/BackgroundDecor.vue';
import PillSelector from '@/components/pills/PillSelector.vue';
import ErrorMessage from '@/components/results/ErrorMessage.vue';
import ResultsSection from '@/components/results/ResultsSection.vue';
import AppointmentPanel from '@/components/settings/AppointmentPanel.vue';
import SettingsPanel from '@/components/settings/SettingsPanel.vue';
import { useDebounce } from '@/composables/use-debounce';
import { useAppointmentStore } from '@/stores/appointment';
import { useWarfarinStore } from '@/stores/warfarin';

// --- Animation State ---
const mounted = ref(false);

// --- Store References ---
const warfarinStore = useWarfarinStore();
const appointmentStore = useAppointmentStore();

const { allowHalf, availablePills, specialDayPattern, results } = storeToRefs(warfarinStore);
const { appointmentToggle, startDate, endDate } = storeToRefs(appointmentStore);

// --- Lifecycle ---
onMounted(async () => {
  setTimeout(() => {
    mounted.value = true;
  }, 100);

  await warfarinStore.initWasm();
  appointmentStore.initializeStartDate();
});

// --- Auto-recalculate on passive setting changes ---
const debouncedRecalculate = useDebounce(() => {
  if (results.value.length > 0) {
    warfarinStore.handleCalculation();
  }
}, 500);

watch(
  [allowHalf, availablePills, specialDayPattern, startDate, endDate],
  debouncedRecalculate,
  { deep: true },
);
</script>

<template>
  <div class="min-h-screen flex items-center justify-center py-10 px-4 sm:px-6 relative font-sans text-slate-800">
    <!-- Background Decor -->
    <BackgroundDecor />

    <div id="app-content" class="w-full max-w-2xl mx-auto relative">
      <!-- Header -->
      <AppHeader :mounted="mounted" />

      <!-- Main Card -->
      <div
        class="glass-panel rounded-3xl p-8 sm:p-10 mb-8 relative overflow-hidden transition-all duration-700 delay-100"
        :class="{ 'translate-y-0 opacity-100': mounted, 'translate-y-8 opacity-0': !mounted }"
      >
        <!-- Step 1: Target Dose Input -->
        <DoseInput />

        <hr class="border-gray-100 my-8">

        <!-- Step 2: Configuration -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
          <PillSelector />
          <SettingsPanel />
        </div>

        <!-- Appointment Date Inputs (Conditional) -->
        <transition name="list">
          <AppointmentPanel v-if="appointmentToggle" />
        </transition>

        <!-- Action Button -->
        <CalculateButton />
      </div>

      <!-- Error Message -->
      <ErrorMessage />

      <!-- Results Section -->
      <ResultsSection />

      <!-- Footer -->
      <AppFooter />
    </div>
  </div>
</template>

<style>
/* --- Design Token Overrides & Global Utilities --- */
:root {
  --ease-apple: cubic-bezier(0.25, 1, 0.5, 1);
  --glass-bg: rgba(255, 255, 255, 0.85);
  --shadow-soft: 0 20px 40px -10px rgba(0, 0, 0, 0.05);
}

body {
  background-color: #f5f5f7;
  -webkit-font-smoothing: antialiased;
}

/* --- Custom Components --- */
.glass-panel {
  background: var(--glass-bg);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  box-shadow: var(--shadow-soft);
}

.input-reset {
  background: transparent;
  border: none;
  outline: none;
  text-align: center;
}

.scale-tap:active {
  transform: scale(0.96);
}

/* --- Transitions --- */
.fade-enter-active,
.fade-leave-active {
  transition:
    opacity 0.4s var(--ease-apple),
    transform 0.4s var(--ease-apple);
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}

.list-enter-active,
.list-leave-active {
  transition: all 0.5s var(--ease-apple);
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(20px);
}
</style>
