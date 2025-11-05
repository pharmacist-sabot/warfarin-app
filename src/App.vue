<script setup>
import { ref, onMounted, computed, watch } from 'vue';

// 1. Import ทุกอย่างจาก Rust/WASM package ของเรา
// อย่าลืมรัน `wasm-pack build ./warfarin_logic --target web` ก่อนนะ
import init, { generate_suggestions_rust } from '../warfarin_logic/pkg/warfarin_logic.js';

// --- State Management ---
const previousDose = ref(null);
const weeklyDose = ref(null);
const allowHalf = ref(true);
const availablePills = ref({ 1: false, 2: true, 3: true, 5: true });
const specialDayPattern = ref('fri-sun');
const displayOrder = ref('sun-sat');
const results = ref([]);
const isLoading = ref(false);
const errorMsg = ref('');
const wasmReady = ref(false);

// --- Appointment State ---
const appointmentToggle = ref(false);
const startDate = ref('');
const endDate = ref('');

// --- Computed Properties ---
const appointmentInfo = computed(() => {
    let daysUntilAppointment = 7;
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
        errorMsg.value = 'กรุณากรอกขนาดยาใหม่ที่ถูกต้อง';
        results.value = [];
        return;
    }
    const selectedPills = Object.keys(availablePills.value).filter(key => availablePills.value[key]);
    if (selectedPills.length === 0) {
        errorMsg.value = 'กรุณาเลือกขนาดยาอย่างน้อย 1 ขนาด';
        results.value = [];
        return;
    }

    isLoading.value = true;
    errorMsg.value = '';
    results.value = [];

    try {
        // 2. เตรียมข้อมูลส่งให้ Rust ในรูปแบบที่กำหนดไว้ใน struct
        const input = {
            weekly_dose: parseFloat(weeklyDose.value),
            allow_half: allowHalf.value,
            available_pills: selectedPills.map(Number),
            special_day_pattern: specialDayPattern.value,
            days_until_appointment: appointmentInfo.value.daysUntilAppointment,
            start_day_of_week: appointmentInfo.value.startDayOfWeek,
        };

        // 3. เรียกฟังก์ชัน Rust!
        const rustResults = await generate_suggestions_rust(input);
        results.value = rustResults;

        if (rustResults.length === 0) {
            errorMsg.value = 'ไม่พบตัวเลือกที่เหมาะสมสำหรับขนาดยาที่ต้องการ';
        }

    } catch (e) {
        console.error("Error calling Rust WASM function:", e);
        errorMsg.value = `เกิดข้อผิดพลาด: ${e}`;
    } finally {
        isLoading.value = false;
    }
};

// --- Dose Adjustment ---
const adjustmentPercentages = [-20, -15, -10, -5, 0, 5, 10, 15, 20];
const setWeeklyDoseAndSuggest = (dose) => {
    const roundedDose = Math.round(dose * 2) / 2;
    weeklyDose.value = roundedDose.toFixed(1);
    // ใช้ nextTick เพื่อให้ UI อัปเดตค่า weeklyDose ก่อนคำนวณ
    // import { nextTick } from 'vue';
    // nextTick(() => { handleCalculation(); });
};

// --- Lifecycle Hook ---
onMounted(async () => {
    try {
        await init();
        wasmReady.value = true;
        console.log('WASM module initialized!');
    } catch (e) {
        console.error("Failed to initialize WASM module", e);
        errorMsg.value = "ไม่สามารถโหลดโมดูลคำนวณได้ กรุณาลองรีเฟรชหน้าใหม่อีกครั้ง";
    }
});

// --- Watchers ---
watch(appointmentToggle, (isToggled) => {
    if (isToggled && !startDate.value) {
        const today = new Date();
        startDate.value = today.toISOString().split('T')[0];
    }
    if (results.value.length > 0) {
        handleCalculation();
    }
});

const debouncedRecalculate = debounce(() => {
    if (results.value.length > 0) {
        handleCalculation();
    }
}, 300);

watch([allowHalf, availablePills, specialDayPattern, startDate, endDate], debouncedRecalculate, { deep: true });

function debounce(fn, delay) {
    let timeoutID = null;
    return function (...args) {
        clearTimeout(timeoutID);
        timeoutID = setTimeout(() => {
            fn(...args);
        }, delay);
    };
}

</script>

<template>
    <div class="app-container">
        <div class="main-card">
            <div class="header-badge">
                <span>Warfarin คิดแป๊ปเดียว !</span>
            </div>
            <h1 class="main-title">โปรแกรมคำนวณขนาดยา Warfarin</h1>
            <p class="powered-by">Powered by Rust 🦀 + Vue.js 💚</p>

            <!-- Canva Embed -->
            <div class="canva-container">
                <iframe loading="lazy" class="canva-iframe"
                    src="https://www.canva.com/design/DAGr4hxyiRI/T8PUUZhf8fgNkFF_EU0iPQ/view?embed"
                    allowfullscreen="allowfullscreen" allow="fullscreen">
                </iframe>
            </div>
            <a href="https://www.canva.com/design/DAGr4hxyiRI/T8PUUZhf8fgNkFF_EU0iPQ/view?utm_content=DAGr4hxyiRI&amp;utm_campaign=designshare&amp;utm_medium=embeds&amp;utm_source=link"
                target="_blank" rel="noopener" class="canva-link">
                Design by Narawit Kamyuang
            </a>

            <hr class="divider">

            <!-- Dose Inputs -->
            <div class="input-grid">
                <div class="form-group">
                    <label for="previousDoseInput">ขนาดยาก่อนหน้า (mg/week)</label>
                    <input type="number" id="previousDoseInput" v-model.number="previousDose" step="0.1" min="0">
                </div>
                <div class="form-group">
                    <label for="weeklyDoseInput">ขนาดยาใหม่ (mg/week)</label>
                    <input type="number" id="weeklyDoseInput" v-model.number="weeklyDose" step="0.1" min="0"
                        class="new-dose-input">
                </div>
                <div class="checkbox-group-vertical">
                    <input type="checkbox" id="allowHalf" v-model="allowHalf">
                    <label for="allowHalf">ให้ใช้ *ครึ่ง* เม็ด</label>
                </div>
            </div>

            <!-- Pill Selection -->
            <div class="option-section">
                <label class="section-label">เลือกขนาดยาที่ใช้คำนวณ</label>
                <div class="pill-selection-container">
                    <div v-for="pill in [1, 2, 3, 5]" :key="pill" class="checkbox-group">
                        <input type="checkbox" :id="`pill-${pill}`" :value="pill" v-model="availablePills[pill]">
                        <label :for="`pill-${pill}`" class="pill-label">
                            <span :class="`pill pill-${pill}`"></span> {{ pill }} mg
                        </label>
                    </div>
                </div>
            </div>

            <hr class="divider">

            <!-- Dose Adjustment -->
            <div v-if="previousDose > 0" class="adjustment-section">
                <h3 class="section-label">ปรับขนาดยาอัตโนมัติ <span class="sub-label">🖱️ (คลิกเพื่อเลือก)</span></h3>
                <div class="adjustment-grid">
                    <button v-for="p in adjustmentPercentages" :key="p"
                        @click="setWeeklyDoseAndSuggest(previousDose * (1 + p / 100))" :class="[
                            'dose-adjust-btn',
                            p < 0 ? 'btn-red' : (p === 0 ? 'btn-blue' : 'btn-green')
                        ]">
                        <div class="btn-percent">{{ p > 0 ? '+' : '' }}{{ p }}%</div>
                        <div class="btn-dose">{{ (previousDose * (1 + p / 100)).toFixed(1) }} mg</div>
                    </button>
                </div>
            </div>

            <!-- Show Button -->
            <div class="button-container">
                <button @click="handleCalculation" :disabled="!wasmReady || isLoading" class="calculate-btn">
                    <span v-if="!wasmReady">กำลังโหลด...</span>
                    <span v-else-if="isLoading">กำลังคำนวณ...</span>
                    <span v-else>แสดงตัวเลือก</span>
                </button>
            </div>

            <hr class="divider">

            <!-- Appointment Section -->
            <div class="option-section">
                <div class="checkbox-group justify-center">
                    <input type="checkbox" id="appointmentToggle" v-model="appointmentToggle">
                    <label for="appointmentToggle" class="font-bold">คำนวณจำนวนยาตามนัด</label>
                </div>
                <p class="help-text">*หากไม่ระบุ จะคำนวณยาที่ใช้ใน 1 สัปดาห์</p>

                <div v-if="appointmentToggle" class="appointment-fields">
                    <div class="input-grid">
                        <div class="form-group">
                            <label for="startDate">วันที่เริ่มต้น (วันที่มา):</label>
                            <input type="date" id="startDate" v-model="startDate">
                        </div>
                        <div>
                            <label for="endDate">วันนัดครั้งถัดไป:</label>
                            <input type="date" id="endDate" v-model="endDate">
                        </div>
                    </div>
                    <div v-if="appointmentDaysText" class="days-result" aria-live="polite">{{ appointmentDaysText }}
                    </div>
                </div>
            </div>

            <hr class="divider">

            <!-- Results Section -->
            <div id="result-container" aria-live="polite">
                <div v-if="errorMsg" class="error-box">{{ errorMsg }}</div>
                <div v-for="(option, index) in results" :key="index" class="result-card">
                    <div class="result-header">ตัวเลือก {{ index + 1 }}: {{ option.description }} (รวม {{
                        option.weekly_dose_actual.toFixed(1) }} mg/สัปดาห์)</div>
                    <div v-html="option.weekly_schedule_html"></div>
                    <div class="result-footer">
                        <div v-html="option.total_pills_message"></div>
                    </div>
                </div>
            </div>

        </div>
    </div>
</template>

<style scoped>
/* --- General Styles & Fonts --- */
@import url('https://fonts.googleapis.com/css2?family=Prompt:wght@300;400;500;700&display=swap');

:global(body) {
    margin: 0;
    background-color: #f3f4f6;
    font-family: 'Prompt', sans-serif;
}

.app-container {
    padding: 1rem;
}

.main-card {
    max-width: 80rem;
    /* ~max-w-5xl */
    margin: 0 auto;
    background-color: white;
    padding: 1.5rem;
    border-radius: 1rem;
    box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
    position: relative;
}

/* --- Header & Title --- */
.main-title {
    font-size: 1.5rem;
    font-weight: 700;
    margin-bottom: 1.5rem;
    text-align: center;
    color: #1e3a8a;
    /* ~text-blue-800 */
    background-color: #eff6ff;
    /* ~bg-blue-50 */
    padding: 1rem;
    border-radius: 0.75rem;
    box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
}

.powered-by {
    text-align: center;
    color: #4b5563;
    margin-top: -1rem;
    margin-bottom: 1.5rem;
    font-size: 0.9rem;
}

.header-badge {
    position: absolute;
    top: 1.5rem;
    right: 1.5rem;
}

.header-badge span {
    background-color: #dbeafe;
    /* ~bg-blue-100 */
    color: #1d4ed8;
    /* ~text-blue-700 */
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.25rem 0.625rem;
    border-radius: 9999px;
}

.divider {
    margin: 1.5rem 0;
    border: none;
    border-top: 1px solid #e5e7eb;
}

/* --- Canva Embed --- */
.canva-container {
    position: relative;
    width: 100%;
    padding-top: 56.25%;
    /* 16:9 Aspect Ratio */
    box-shadow: 0 2px 8px 0 rgba(63, 69, 81, 0.16);
    margin-top: 1.6em;
    margin-bottom: 0.9em;
    overflow: hidden;
    border-radius: 8px;
}

.canva-iframe {
    position: absolute;
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    border: none;
}

.canva-link {
    display: block;
    text-align: center;
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.5rem;
    text-decoration: none;
}

.canva-link:hover {
    color: #2563eb;
}

/* --- Forms & Inputs --- */
.input-grid {
    display: grid;
    grid-template-columns: 1fr;
    gap: 1rem;
}

@media (min-width: 768px) {
    .input-grid {
        grid-template-columns: repeat(3, 1fr);
    }
}

.form-group {
    display: flex;
    flex-direction: column;
}

.form-group label,
.checkbox-group label,
.section-label {
    color: #374151;
    font-size: 0.875rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
}

.form-group input[type="number"],
.form-group input[type="date"] {
    width: 100%;
    border: 1px solid #d1d5db;
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    outline: none;
    transition: box-shadow 0.2s;
}

.form-group input:focus {
    box-shadow: 0 0 0 2px #3b82f6;
}

.new-dose-input {
    background-color: #ffedd5;
    /* ~bg-orange-100 */
}

.new-dose-input:focus {
    box-shadow: 0 0 0 2px #f97316;
}

/* --- Checkboxes & Radios --- */
.checkbox-group,
.checkbox-group-vertical {
    display: flex;
    align-items: center;
}

.checkbox-group-vertical {
    padding-top: 1.75rem;
}

.checkbox-group input[type="checkbox"] {
    height: 1.25rem;
    width: 1.25rem;
    accent-color: #f97316;
}

.checkbox-group label,
.checkbox-group-vertical label {
    margin-left: 0.5rem;
    margin-bottom: 0;
    font-weight: 700;
}

/* --- Pill Selection --- */
.option-section {
    margin: 1rem 0;
    padding-top: 0.5rem;
}

.section-label {
    display: block;
    text-align: center;
    margin-bottom: 0.5rem;
}

.pill-selection-container {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    flex-wrap: wrap;
}

.pill-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
}

/* --- Dose Adjustment Buttons --- */
.adjustment-section {
    margin: 1rem 0;
}

.sub-label {
    font-size: 0.875rem;
    font-weight: 400;
}

.adjustment-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.5rem;
}

@media (min-width: 768px) {
    .adjustment-grid {
        grid-template-columns: repeat(5, 1fr);
    }
}

@media (min-width: 1024px) {
    .adjustment-grid {
        grid-template-columns: repeat(9, 1fr);
    }
}

.dose-adjust-btn {
    padding: 0.5rem;
    border-radius: 0.5rem;
    transition: background-color 0.15s ease-in-out;
    border: 1px solid #e5e7eb;
    box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
    cursor: pointer;
}

.dose-adjust-btn:focus {
    outline: 2px solid #4f46e5;
    outline-offset: 2px;
}

.btn-percent {
    font-weight: 700;
    font-size: 1.125rem;
}

.btn-dose {
    font-size: 0.875rem;
}

.btn-red {
    background-color: #fee2e2;
    color: #991b1b;
}

.btn-red:hover {
    background-color: #fecaca;
}

.btn-blue {
    background-color: #dbeafe;
    color: #1e40af;
}

.btn-blue:hover {
    background-color: #bfdbfe;
}

.btn-green {
    background-color: #d1fae5;
    color: #065f46;
}

.btn-green:hover {
    background-color: #a7f3d0;
}

/* --- Calculate Button --- */
.button-container {
    display: flex;
    justify-content: center;
}

.calculate-btn {
    background-color: #2563eb;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
    width: 100%;
    border: none;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.15s ease-in-out;
}

@media (min-width: 768px) {
    .calculate-btn {
        width: auto;
    }
}

.calculate-btn:hover {
    background-color: #1d4ed8;
}

.calculate-btn:disabled {
    background-color: #9ca3af;
    cursor: not-allowed;
}

/* --- Appointment --- */
.font-bold {
    font-weight: 700;
}

.justify-center {
    justify-content: center;
}

.help-text {
    text-align: center;
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
}

.appointment-fields {
    margin-top: 1rem;
}

.days-result {
    text-align: center;
    font-size: 1.125rem;
    font-weight: 600;
    margin-top: 1rem;
    color: #374151;
}

/* --- Results --- */
#result-container {
    margin-top: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.error-box {
    color: #991b1b;
    background-color: #fee2e2;
    text-align: center;
    font-weight: 700;
    padding: 1rem;
    border-radius: 0.5rem;
}

.result-card {
    border: 1px solid #e5e7eb;
    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #f9fafb;
    box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);
}

.result-header {
    font-weight: 600;
    color: #1d4ed8;
    margin-bottom: 0.5rem;
}

.result-footer {
    margin-top: 1rem;
    font-size: 0.875rem;
    border-top: 1px solid #e5e7eb;
    padding-top: 0.5rem;
    color: #374151;
}

/* --- Footer --- */
.footer {
    text-align: right;
    font-size: 0.75rem;
    color: #9ca3af;
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid #e5e7eb;
}

/* --- Pill Styles (Global) --- */
:global(.pill) {
    width: 24px;
    height: 24px;
    border-radius: 9999px;
    display: inline-block;
    margin: 2px;
}

:global(.pill-1) {
    background-color: #E5E7EB;
    border: 1px solid #D1D5DB;
}

:global(.pill-2) {
    background-color: orange;
}

:global(.pill-3) {
    background-color: skyblue;
}

:global(.pill-5) {
    background-color: pink;
}

:global(.pill-half-left) {
    clip-path: inset(0 50% 0 0);
}
</style>
