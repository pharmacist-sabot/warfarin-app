<script setup>
import { ref, onMounted, computed, watch, nextTick } from 'vue';
import init, { generate_suggestions_rust } from '../warfarin_logic/pkg/warfarin_logic.js';

// --- State Management ---
const mounted = ref(false); // For entrance animations
const weeklyDose = ref(null);
const allowHalf = ref(true);
const availablePills = ref({ 1: false, 2: true, 3: true, 5: true });
const specialDayPattern = ref('fri-sun'); // Keep internal logic
const results = ref([]);
const loading = ref(false);
const errorMsg = ref('');
const wasmReady = ref(false);

// --- Pill Config for UI ---
const pillTypes = [
    { mg: 1, colorClass: 'bg-gray-300' },
    { mg: 2, colorClass: 'bg-orange-300' },
    { mg: 3, colorClass: 'bg-sky-400' },
    { mg: 5, colorClass: 'bg-pink-400' }
];

// --- Appointment State ---
const appointmentToggle = ref(false);
const startDate = ref('');
const endDate = ref('');

// --- Computed Properties (Logic preserved) ---
const appointmentInfo = computed(() => {
    let daysUntilAppointment = 7; // Default
    let startDayOfWeek = 0; // 0=Mon, ..., 6=Sun

    if (appointmentToggle.value && startDate.value && endDate.value) {
        const startDt = new Date(startDate.value);
        const endDt = new Date(endDate.value);
        if (endDt > startDt) {
            const timeDiff = endDt.getTime() - startDt.getTime();
            daysUntilAppointment = Math.max(1, Math.round(timeDiff / (1000 * 3600 * 24)));

            const jsDay = startDt.getDay(); // JS: 0=Sun, 1=Mon...
            startDayOfWeek = (jsDay === 0) ? 6 : jsDay - 1;
        }
    }
    return { daysUntilAppointment, startDayOfWeek };
});

const appointmentDaysText = computed(() => {
    if (appointmentToggle.value && appointmentInfo.value.daysUntilAppointment > 0 && startDate.value && endDate.value) {
        return `คำนวณสำหรับ ${appointmentInfo.value.daysUntilAppointment} วัน`;
    }
    return '';
});

// --- Logic ---
const handleCalculation = async () => {
    if (weeklyDose.value === null || weeklyDose.value < 0) {
        errorMsg.value = 'กรุณากรอกขนาดยาเป้าหมายให้ถูกต้อง';
        results.value = [];
        return;
    }

    // Sort keys to match UI logic (descending generally, but logic handles it)
    const selectedPills = Object.keys(availablePills.value)
        .filter(key => availablePills.value[key]);

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
        const input = {
            weekly_dose: parseFloat(weeklyDose.value),
            allow_half: allowHalf.value,
            available_pills: selectedPills.map(Number),
            special_day_pattern: specialDayPattern.value,
            days_until_appointment: appointmentInfo.value.daysUntilAppointment,
            start_day_of_week: appointmentInfo.value.startDayOfWeek,
        };

        const rustResults = await generate_suggestions_rust(input);
        results.value = rustResults;

        if (rustResults.length === 0) {
            errorMsg.value = 'ไม่พบตัวเลือกที่เหมาะสมสำหรับเงื่อนไขนี้ (ลองอนุญาตให้ใช้ครึ่งเม็ด)';
        } else {
            nextTick(() => {
                const resultsEl = document.getElementById('results-section');
                if (resultsEl) resultsEl.scrollIntoView({ behavior: 'smooth', block: 'start' });
            });
        }

    } catch (e) {
        console.error("Error calling Rust WASM function:", e);
        errorMsg.value = `เกิดข้อผิดพลาดในการคำนวณ: ${e}`;
    } finally {
        loading.value = false;
    }
};

// --- Dose Adjustment (New UI Style) ---
const adjustDose = (percent) => {
    if (!weeklyDose.value) weeklyDose.value = 0;
    // Base adjustment on current input to act like a calculator
    let current = parseFloat(weeklyDose.value);
    if (isNaN(current)) current = 0;

    if (percent === 0) {
        // "Same" - logic implies keeping it, but maybe rounding it?
        // In this context, just ensure it's a number
    } else {
        let next = current * (1 + percent / 100);
        weeklyDose.value = Math.round(next * 2) / 2; // Round to nearest 0.5
    }
};

const togglePill = (mg) => {
    availablePills.value[mg] = !availablePills.value[mg];
};

// --- Lifecycle & Watchers ---
onMounted(async () => {
    // Animation trigger
    setTimeout(() => { mounted.value = true; }, 100);

    try {
        await init();
        wasmReady.value = true;
        console.log('WASM module initialized!');
    } catch (e) {
        console.error("Failed to initialize WASM module", e);
        errorMsg.value = "ไม่สามารถโหลดระบบคำนวณได้ กรุณารีเฟรชหน้าเว็บ";
    }

    // Initialize dates if needed
    const today = new Date();
    startDate.value = today.toISOString().split('T')[0];
});

// Auto re-calculate when passive settings change (if results exist)
function debounce(fn, delay) {
    let timeoutID = null;
    return function (...args) {
        clearTimeout(timeoutID);
        timeoutID = setTimeout(() => fn(...args), delay);
    };
}

const debouncedRecalculate = debounce(() => {
    if (results.value.length > 0) handleCalculation();
}, 500);

watch([allowHalf, availablePills, specialDayPattern, startDate, endDate], debouncedRecalculate, { deep: true });
</script>

<template>
    <div class="min-h-screen flex items-center justify-center py-10 px-4 sm:px-6 relative font-sans text-slate-800">

        <!-- Background Decor -->
        <div class="fixed top-0 left-0 w-full h-full overflow-hidden -z-10 pointer-events-none">
            <div
                class="absolute top-[-10%] right-[-5%] w-[500px] h-[500px] bg-blue-100 rounded-full blur-[120px] opacity-60">
            </div>
            <div
                class="absolute bottom-[-10%] left-[-10%] w-[600px] h-[600px] bg-orange-50 rounded-full blur-[100px] opacity-60">
            </div>
        </div>

        <div id="app-content" class="w-full max-w-2xl mx-auto relative">

            <!-- Header -->
            <header class="text-center mb-10 transition-all duration-700 ease-out transform"
                :class="{ 'translate-y-0 opacity-100': mounted, '-translate-y-4 opacity-0': !mounted }">
                <div
                    class="inline-flex items-center justify-center space-x-2 bg-white px-4 py-1.5 rounded-full shadow-sm border border-gray-100 mb-6">
                    <span class="w-2 h-2 rounded-full"
                        :class="wasmReady ? 'bg-green-500 animate-pulse' : 'bg-red-400'"></span>
                    <span class="text-xs font-medium text-gray-500 tracking-wide uppercase">Dose Calculator</span>
                </div>
                <h1 class="text-4xl sm:text-5xl font-bold tracking-tight text-gray-900 mb-2 font-display">
                    Warfarin
                </h1>
                <p class="text-gray-500 text-lg font-light">
                    Precision Dosing Assistant
                </p>
            </header>

            <!-- Main Card -->
            <div class="glass-panel rounded-3xl p-8 sm:p-10 mb-8 relative overflow-hidden transition-all duration-700 delay-100"
                :class="{ 'translate-y-0 opacity-100': mounted, 'translate-y-8 opacity-0': !mounted }">

                <!-- Step 1: Target Dose Input -->
                <div class="mb-10 text-center">
                    <label class="block text-sm font-semibold text-gray-400 uppercase tracking-wider mb-4">Target Weekly
                        Dose (mg)</label>
                    <div class="relative inline-block group">
                        <input type="number" v-model.number="weeklyDose"
                            class="input-reset text-6xl sm:text-7xl font-light text-gray-900 placeholder-gray-200 w-full max-w-[300px] border-b-2 border-transparent focus:border-blue-500 transition-colors duration-300 pb-2"
                            placeholder="0" step="0.5" @keydown.enter="handleCalculation">
                        <span class="text-xl text-gray-400 absolute top-4 -right-8 font-light">mg</span>
                    </div>

                    <!-- Quick Adjustments -->
                    <div class="flex justify-center gap-3 mt-6">
                        <button @click="adjustDose(-10)"
                            class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-red-50 hover:text-red-600 transition-colors scale-tap">-10%</button>
                        <button @click="adjustDose(0)"
                            class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-gray-100 transition-colors scale-tap">Same</button>
                        <button @click="adjustDose(10)"
                            class="px-4 py-2 rounded-xl bg-gray-50 text-gray-600 text-sm font-medium hover:bg-green-50 hover:text-green-600 transition-colors scale-tap">+10%</button>
                    </div>
                </div>

                <hr class="border-gray-100 my-8">

                <!-- Step 2: Configuration -->
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">

                    <!-- Pill Availability -->
                    <div>
                        <label class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-4">Available
                            Pills</label>
                        <div class="flex flex-wrap gap-3">
                            <button v-for="pill in pillTypes" :key="pill.mg" @click="togglePill(pill.mg)"
                                class="relative group flex items-center gap-3 pl-2 pr-4 py-2 rounded-full border transition-all duration-300 scale-tap"
                                :class="availablePills[pill.mg] ? 'bg-white border-gray-200 shadow-md ring-1 ring-black/5' : 'bg-gray-50 border-transparent opacity-60 grayscale'">
                                <!-- Pill Visual -->
                                <div class="w-8 h-8 rounded-full shadow-inner flex items-center justify-center text-[10px] font-bold text-white/90"
                                    :class="pill.colorClass">
                                    {{ pill.mg }}
                                </div>
                                <span class="text-sm font-medium text-gray-700 group-hover:text-gray-900">{{ pill.mg }}
                                    mg</span>

                                <!-- Checkmark Icon -->
                                <div v-if="availablePills[pill.mg]"
                                    class="absolute top-0 right-0 -mt-1 -mr-1 bg-green-500 text-white rounded-full p-0.5 shadow-sm">
                                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24"
                                        fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round"
                                        stroke-linejoin="round">
                                        <polyline points="20 6 9 17 4 12"></polyline>
                                    </svg>
                                </div>
                            </button>
                        </div>
                    </div>

                    <!-- Settings & Appointment -->
                    <div>
                        <label
                            class="block text-xs font-bold text-gray-400 uppercase tracking-wider mb-4">Settings</label>
                        <div class="space-y-3">
                            <!-- Allow Half Pills -->
                            <label
                                class="flex items-center justify-between p-3 rounded-xl bg-gray-50 hover:bg-white border border-transparent hover:border-gray-100 transition-all cursor-pointer group">
                                <span class="text-sm text-gray-600 group-hover:text-gray-900">Allow Half Pills
                                    (ครึ่งเม็ด)</span>
                                <div
                                    class="relative inline-block w-10 mr-2 align-middle select-none transition duration-200 ease-in">
                                    <input type="checkbox" v-model="allowHalf"
                                        class="toggle-checkbox absolute block w-5 h-5 rounded-full bg-white border-4 appearance-none cursor-pointer transition-transform duration-300 ease-in-out"
                                        :class="allowHalf ? 'translate-x-5 border-blue-500' : 'translate-x-0 border-gray-300'" />
                                    <div class="toggle-label block overflow-hidden h-5 rounded-full bg-gray-200 cursor-pointer transition-colors duration-300"
                                        :class="allowHalf ? 'bg-blue-200' : ''"></div>
                                </div>
                            </label>

                            <!-- Appointment Toggle -->
                            <label
                                class="flex items-center justify-between p-3 rounded-xl bg-gray-50 hover:bg-white border border-transparent hover:border-gray-100 transition-all cursor-pointer group">
                                <span class="text-sm text-gray-600 group-hover:text-gray-900">Appointment Mode</span>
                                <div
                                    class="relative inline-block w-10 mr-2 align-middle select-none transition duration-200 ease-in">
                                    <input type="checkbox" v-model="appointmentToggle"
                                        class="toggle-checkbox absolute block w-5 h-5 rounded-full bg-white border-4 appearance-none cursor-pointer transition-transform duration-300 ease-in-out"
                                        :class="appointmentToggle ? 'translate-x-5 border-blue-500' : 'translate-x-0 border-gray-300'" />
                                    <div class="toggle-label block overflow-hidden h-5 rounded-full bg-gray-200 cursor-pointer transition-colors duration-300"
                                        :class="appointmentToggle ? 'bg-blue-200' : ''"></div>
                                </div>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- Appointment Date Inputs (Conditional) -->
                <transition name="list">
                    <div v-if="appointmentToggle" class="mt-6 bg-blue-50/50 rounded-2xl p-5 border border-blue-100">
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-xs font-semibold text-blue-800 mb-1">Start Date
                                    (วันที่มา)</label>
                                <input type="date" v-model="startDate"
                                    class="w-full bg-white border border-blue-200 text-gray-700 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 outline-none">
                            </div>
                            <div>
                                <label class="block text-xs font-semibold text-blue-800 mb-1">Next Appointment
                                    (วันนัด)</label>
                                <input type="date" v-model="endDate"
                                    class="w-full bg-white border border-blue-200 text-gray-700 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block p-2.5 outline-none">
                            </div>
                        </div>
                        <div v-if="appointmentDaysText" class="mt-3 text-center text-sm font-medium text-blue-600">
                            {{ appointmentDaysText }}
                        </div>
                    </div>
                </transition>

                <!-- Action Button -->
                <div class="mt-12">
                    <button @click="handleCalculation" :disabled="loading || !wasmReady"
                        class="w-full py-4 rounded-2xl bg-[#0071E3] text-white font-semibold text-lg shadow-lg shadow-blue-500/30 hover:shadow-blue-500/50 hover:bg-[#0077ED] transition-all transform active:scale-[0.98] disabled:opacity-70 disabled:cursor-not-allowed flex items-center justify-center gap-2">
                        <span v-if="loading || !wasmReady" class="animate-spin">
                            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"
                                fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                                stroke-linejoin="round">
                                <path d="M21 12a9 9 0 1 1-6.219-8.56" />
                            </svg>
                        </span>
                        <span v-else>Generate Regimen</span>
                    </button>
                </div>
            </div>

            <!-- Error Message -->
            <transition name="fade">
                <div v-if="errorMsg"
                    class="text-center p-6 bg-red-50 text-red-600 rounded-2xl mb-8 border border-red-100 shadow-sm">
                    {{ errorMsg }}
                </div>
            </transition>

            <!-- Results Section -->
            <div v-if="results.length > 0" id="results-section" class="mb-20 scroll-mt-6">
                <div class="flex items-center justify-between mb-6 px-2">
                    <h2 class="text-xl font-semibold text-gray-900">Suggested Regimens</h2>
                    <span class="text-xs font-medium bg-gray-200 text-gray-600 px-2 py-1 rounded-md">{{ results.length
                        }} Options</span>
                </div>

                <transition-group name="list" tag="div" class="space-y-4">
                    <div v-for="(option, index) in results" :key="index"
                        class="glass-card-white bg-white rounded-2xl p-6 shadow-sm border border-gray-100 hover:shadow-md transition-shadow">

                        <!-- Header of Result Card -->
                        <div
                            class="flex flex-col sm:flex-row sm:items-center justify-between mb-4 pb-4 border-b border-gray-50">
                            <div>
                                <div class="text-xs font-bold text-blue-600 uppercase tracking-wide mb-1">Option {{
                                    index + 1 }}</div>
                                <div class="text-gray-900 font-medium text-lg">{{ option.description }}</div>
                            </div>
                            <div class="mt-2 sm:mt-0 text-right">
                                <div class="text-2xl font-bold text-gray-900">{{ option.weekly_dose_actual.toFixed(1) }}
                                    <span class="text-sm font-normal text-gray-400">mg/wk</span></div>
                            </div>
                        </div>

                        <!-- Schedule Grid (Rendered from Rust HTML) -->
                        <div class="rust-html-content" v-html="option.weekly_schedule_html"></div>

                        <!-- Footer Info -->
                        <div
                            class="mt-4 pt-4 border-t border-gray-50 text-sm text-gray-600 bg-gray-50/50 rounded-lg p-3">
                            <div v-html="option.total_pills_message"></div>
                        </div>
                    </div>
                </transition-group>
            </div>

            <!-- Footer -->
            <footer class="text-center text-gray-400 text-xs pb-10">
                <p>Powered by <span class="text-orange-400 font-semibold">Rust WASM</span> + <span
                        class="text-green-500 font-semibold">Vue 3</span></p>
                <p class="mt-2 opacity-60">Medical Disclaimer: Tool for professional guidance only.</p>
            </footer>

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
    background-color: #F5F5F7;
    /* Apple Gray Background */
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
    transition: opacity 0.4s var(--ease-apple), transform 0.4s var(--ease-apple);
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

/* --- Rust HTML Content Overrides --- */
/* This section styles the HTML returned by Rust to match the new design */
.rust-html-content .grid {
    display: grid;
    /* Tailwind handles cols via classes passed from Rust */
}

/* Override the day cards from Rust */
.rust-html-content .rounded-lg {
    border-radius: 0.75rem !important;
    /* rounded-xl */
    border-color: #F3F4F6 !important;
    /* border-gray-100 */
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
    background-color: white !important;
}

/* Stop days styling override */
.rust-html-content .bg-gray-100 {
    background-color: #F9FAFB !important;
    /* gray-50 */
    opacity: 0.7;
}

/* Pill Styling Overrides to match new design */
.pill {
    display: inline-block;
    width: 28px !important;
    height: 28px !important;
    border-radius: 9999px;
    box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);
    margin: 2px;
    vertical-align: middle;
}

.pill-1 {
    background-color: #D1D5DB;
    border: 1px solid #E5E7EB;
}

/* gray-300 */
.pill-2 {
    background-color: #FDBA74;
}

/* orange-300 */
.pill-3 {
    background-color: #38BDF8;
}

/* sky-400 */
.pill-5 {
    background-color: #F472B6;
}

/* pink-400 */

/* Half pill clipping */
.pill-half-left {
    clip-path: polygon(0 0, 50% 0, 50% 100%, 0% 100%);
}
</style>
