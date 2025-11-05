use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

const FLOAT_TOLERANCE: f64 = 0.01;
const ABSOLUTE_MAX_DAILY_DOSE: f64 = 15.0;
const DOSE_MULTIPLIER_LIMIT: f64 = 2.5;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum SpecialDayPattern {
    #[serde(rename = "fri-sun")]
    FriSun,
    #[serde(rename = "mon-wed-fri")]
    MonWedFri,
}

#[derive(Deserialize, Debug)]
pub struct CalculationInput {
    pub weekly_dose: f64,
    pub allow_half: bool,
    pub available_pills: Vec<u8>,
    pub special_day_pattern: SpecialDayPattern,
    pub days_until_appointment: u32,
    pub start_day_of_week: u8, // 0=Mon, 1=Tue, ..., 6=Sun
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pill {
    pub mg: u8,
    pub count: u8,
    pub half: bool,
}

#[derive(Debug, Clone)]
enum OptionType {
    Uniform(Vec<Pill>),
    NonUniform(Vec<Vec<Pill>>),
}

#[derive(Debug, Clone)]
struct DosageOption {
    option_type: OptionType,
    weekly_dose_actual: f64,
    base_dose: f64,
    special_dose: f64,
    num_stop_days: u8,
    stop_days: Vec<usize>,
    num_special_days: u8,
    special_days: Vec<usize>,
    priority: u8, // 0 for uniform, 1 for non-uniform
    half_pill_complexity: usize,
    pill_color_count: usize,
    total_pill_objects: u32,
}

#[derive(Serialize)]
pub struct FinalOutput {
    pub description: String,
    pub weekly_dose_actual: f64,
    pub weekly_schedule_html: String,
    pub total_pills_message: String,
}

#[wasm_bindgen]
pub fn generate_suggestions_rust(input_js: JsValue) -> Result<JsValue, JsValue> {
    let mut input: CalculationInput =
        serde_wasm_bindgen::from_value(input_js).map_err(|e| e.to_string())?;

    input.available_pills.sort_unstable_by(|a, b| b.cmp(a));

    let weekly_dose_target = input.weekly_dose;
    if weekly_dose_target < 0.0 {
        return Ok(serde_wasm_bindgen::to_value(&Vec::<FinalOutput>::new())?);
    }

    let mut options: Vec<DosageOption> = Vec::new();
    let mut seen_options: HashSet<String> = HashSet::new();

    // --- Case 1: Uniform dose ---
    let daily_dose_target = weekly_dose_target / 7.0;
    if daily_dose_target >= 0.0 {
        let min_pills = if daily_dose_target < FLOAT_TOLERANCE {
            0
        } else {
            1
        };
        let daily_combos = find_comb(
            daily_dose_target,
            &input.available_pills,
            input.allow_half,
            min_pills,
            4,
        );
        for combo in daily_combos {
            let actual_weekly_dose = combo
                .iter()
                .map(|p| p.mg as f64 * p.count as f64 * if p.half { 0.5 } else { 1.0 })
                .sum::<f64>()
                * 7.0;
            if (actual_weekly_dose - weekly_dose_target).abs() < FLOAT_TOLERANCE {
                let key = format!("uniform-{:?}", &combo);
                if seen_options.insert(key) {
                    options.push(DosageOption::new(
                        OptionType::Uniform(combo),
                        actual_weekly_dose,
                    ));
                }
            }
        }
    }

    // --- Case 2: Non-uniform doses ---
    for num_stop_days in 0..=3 {
        for num_special_days in 0..=(3 - num_stop_days) {
            let normal_days_count = 7 - num_stop_days - num_special_days;
            if normal_days_count == 7 {
                continue;
            }

            let (stop_days, special_days) =
                get_day_indices(num_stop_days, num_special_days, input.special_day_pattern);

            for i in 1..=(ABSOLUTE_MAX_DAILY_DOSE * 2.0) as i32 {
                let base_dose = i as f64 / 2.0;
                let normal_day_combos =
                    find_comb(base_dose, &input.available_pills, input.allow_half, 1, 4);
                if normal_day_combos.is_empty() {
                    continue;
                }

                let remaining_dose = weekly_dose_target - (base_dose * normal_days_count as f64);

                if num_special_days == 0 {
                    if remaining_dose.abs() > FLOAT_TOLERANCE {
                        continue;
                    }
                    add_non_uniform_option(
                        &mut options,
                        &mut seen_options,
                        &input,
                        base_dose,
                        &normal_day_combos,
                        0.0,
                        &[vec![]],
                        &stop_days,
                        &special_days,
                    );
                } else {
                    if remaining_dose < FLOAT_TOLERANCE {
                        continue;
                    }
                    let special_day_dose_target = remaining_dose / num_special_days as f64;
                    if (special_day_dose_target - base_dose).abs() < FLOAT_TOLERANCE
                        || special_day_dose_target <= 0.0
                    {
                        continue;
                    }
                    if special_day_dose_target > ABSOLUTE_MAX_DAILY_DOSE
                        || special_day_dose_target > base_dose * DOSE_MULTIPLIER_LIMIT
                    {
                        continue;
                    }

                    let special_day_combos = find_comb(
                        special_day_dose_target,
                        &input.available_pills,
                        input.allow_half,
                        1,
                        4,
                    );
                    if special_day_combos.is_empty() {
                        continue;
                    }

                    add_non_uniform_option(
                        &mut options,
                        &mut seen_options,
                        &input,
                        base_dose,
                        &normal_day_combos,
                        special_day_dose_target,
                        &special_day_combos,
                        &stop_days,
                        &special_days,
                    );
                }
            }
        }
    }

    // --- Sort options by complexity ---
    options.sort_by(|a, b| {
        a.half_pill_complexity
            .cmp(&b.half_pill_complexity)
            .then_with(|| a.priority.cmp(&b.priority))
            .then_with(|| {
                (a.num_stop_days + a.num_special_days).cmp(&(b.num_stop_days + b.num_special_days))
            })
            .then_with(|| a.pill_color_count.cmp(&b.pill_color_count))
            .then_with(|| a.total_pill_objects.cmp(&b.total_pill_objects))
    });

    // --- Render final output for Vue ---
    let final_results: Vec<FinalOutput> = options
        .iter()
        .take(30)
        .map(|opt| render_option(opt, &input))
        .collect();

    Ok(serde_wasm_bindgen::to_value(&final_results)?)
}

// --- Combination Finding Logic ---

/// Finds combinations of pills for a target dose using recursion.
fn find_comb(
    target: f64,
    available_pills: &[u8],
    allow_half: bool,
    min_pills: u8,
    max_pills: u8,
) -> Vec<Vec<Pill>> {
    if target < FLOAT_TOLERANCE {
        return if min_pills == 0 { vec![vec![]] } else { vec![] };
    }
    let mut results = Vec::new();
    let mut current_combo = Vec::new();
    find_recursive(
        target,
        available_pills,
        allow_half,
        max_pills,
        0.0,
        &mut current_combo,
        0,
        &mut results,
    );

    let mut unique_combos = HashSet::new();
    results
        .into_iter()
        .filter_map(|combo| {
            if combo.len() < min_pills as usize {
                return None;
            }
            let mut half_counts = HashMap::new();
            for p in &combo {
                if p.half {
                    *half_counts.entry(p.mg).or_insert(0) += 1;
                }
            }
            if half_counts.values().any(|&count| count > 1) {
                return None;
            }

            let aggregated = aggregate_combo(combo);
            let mut key_parts: Vec<String> = aggregated
                .iter()
                .map(|p| format!("{}-{}-{}", p.mg, p.count, p.half))
                .collect();
            key_parts.sort();
            let key = key_parts.join("|");

            if unique_combos.insert(key) {
                Some(aggregated)
            } else {
                None
            }
        })
        .collect()
}

// The recursive part of find_comb
fn find_recursive(
    target: f64,
    available_pills: &[u8],
    allow_half: bool,
    max_pills: u8,
    current_dose: f64,
    current_combo: &mut Vec<Pill>,
    pill_index: usize,
    results: &mut Vec<Vec<Pill>>,
) {
    if current_combo.len() > max_pills as usize || current_dose > target + FLOAT_TOLERANCE {
        return;
    }

    if (current_dose - target).abs() < FLOAT_TOLERANCE {
        results.push(current_combo.clone());
        return;
    }
    if pill_index >= available_pills.len() {
        return;
    }

    let pill_mg = available_pills[pill_index];

    current_combo.push(Pill {
        mg: pill_mg,
        count: 1,
        half: false,
    });
    find_recursive(
        target,
        available_pills,
        allow_half,
        max_pills,
        current_dose + pill_mg as f64,
        current_combo,
        pill_index,
        results,
    );
    current_combo.pop();

    if allow_half {
        current_combo.push(Pill {
            mg: pill_mg,
            count: 1,
            half: true,
        });
        find_recursive(
            target,
            available_pills,
            allow_half,
            max_pills,
            current_dose + pill_mg as f64 / 2.0,
            current_combo,
            pill_index,
            results,
        );
        current_combo.pop();
    }

    find_recursive(
        target,
        available_pills,
        allow_half,
        max_pills,
        current_dose,
        current_combo,
        pill_index + 1,
        results,
    );
}

// Aggregates a list like [Pill(5mg), Pill(5mg)] into [Pill(5mg, count: 2)]
fn aggregate_combo(combo: Vec<Pill>) -> Vec<Pill> {
    let mut aggregated: HashMap<(u8, bool), u8> = HashMap::new();
    for pill in combo {
        *aggregated.entry((pill.mg, pill.half)).or_insert(0) += 1;
    }
    let mut result: Vec<Pill> = aggregated
        .into_iter()
        .map(|((mg, half), count)| Pill { mg, count, half })
        .collect();
    result.sort_by_key(|p| p.mg);
    result
}

// --- Option Generation and Management ---

fn add_non_uniform_option(
    options: &mut Vec<DosageOption>,
    seen_options: &mut HashSet<String>,
    input: &CalculationInput,
    base_dose: f64,
    normal_day_combos: &[Vec<Pill>],
    special_day_dose_target: f64,
    special_day_combos: &[Vec<Pill>],
    stop_days: &[usize],
    special_days: &[usize],
) {
    for n_combo in normal_day_combos {
        for s_combo in special_day_combos {
            let mut combo_weekly = vec![Vec::new(); 7];
            let mut actual_weekly_dose = 0.0;

            for i in 0..7 {
                let combo_for_day: &[Pill] = if stop_days.contains(&i) {
                    &[]
                } else if special_days.contains(&i) {
                    s_combo.as_slice()
                } else {
                    n_combo.as_slice()
                };

                combo_weekly[i] = combo_for_day.to_vec();
                actual_weekly_dose += combo_for_day
                    .iter()
                    .map(|p| p.mg as f64 * p.count as f64 * if p.half { 0.5 } else { 1.0 })
                    .sum::<f64>();
            }

            if (actual_weekly_dose - input.weekly_dose).abs() < FLOAT_TOLERANCE {
                let key = format!("non-uniform-{:?}", combo_weekly);
                if seen_options.insert(key) {
                    let opt_type = OptionType::NonUniform(combo_weekly);
                    let mut new_option = DosageOption::new(opt_type, actual_weekly_dose);
                    new_option.base_dose = base_dose;
                    new_option.special_dose = special_day_dose_target;
                    new_option.num_stop_days = stop_days.len() as u8;
                    new_option.stop_days = stop_days.to_vec();
                    new_option.num_special_days = special_days.len() as u8;
                    new_option.special_days = special_days.to_vec();
                    options.push(new_option);
                }
            }
        }
    }
}

// --- Rendering and Formatting ---

fn render_option(option: &DosageOption, input: &CalculationInput) -> FinalOutput {
    let days_name = ["จ.", "อ.", "พ.", "พฤ.", "ศ.", "ส.", "อา."];
    let description = option.get_description(&days_name);

    let display_order = [6, 0, 1, 2, 3, 4, 5]; // Sun-Sat default
    let weekly_schedule_html = format!(
        "<div class=\"grid grid-cols-4 sm:grid-cols-7 gap-2 mt-4\">{}</div>",
        display_order
            .iter()
            .map(|&day_idx| {
                let combo = match &option.option_type {
                    OptionType::Uniform(c) => c,
                    OptionType::NonUniform(cw) => &cw[day_idx],
                };
                let day_type = if option.stop_days.contains(&day_idx) {
                    "stop"
                } else if option.special_days.contains(&day_idx) {
                    "special"
                } else {
                    "normal"
                };
                render_day(day_idx, combo, day_type, &days_name)
            })
            .collect::<String>()
    );

    let total_pills_header = format!("รวมยาถึงวันนัด ({} วัน):", input.days_until_appointment);
    let pills_needed_message = calculate_total_pills(
        option,
        input.days_until_appointment,
        input.start_day_of_week,
    );

    FinalOutput {
        description,
        weekly_dose_actual: option.weekly_dose_actual,
        weekly_schedule_html,
        total_pills_message: format!(
            "<span class=\"font-bold\">{}</span><br>{}",
            total_pills_header, pills_needed_message
        ),
    }
}

fn render_day(idx: usize, combo: &[Pill], day_type: &str, days_name: &[&str]) -> String {
    let day_colors = [
        "bg-yellow-100",
        "bg-pink-100",
        "bg-green-100",
        "bg-orange-200",
        "bg-sky-100",
        "bg-purple-100",
        "bg-red-200",
    ];
    let day_dose: f64 = combo
        .iter()
        .map(|p| p.mg as f64 * p.count as f64 * if p.half { 0.5 } else { 1.0 })
        .sum();

    let mut visual_pills = String::new();
    let mut text_pills_arr = Vec::new();
    for p in combo {
        let pill_class = format!("pill pill-{}", p.mg);
        for _ in 0..p.count {
            visual_pills.push_str(&format!(
                "<span class=\"{} {}\"></span>",
                pill_class,
                if p.half { "pill-half-left" } else { "" }
            ));
        }
        if p.count > 0 {
            let pill_count_text = if p.half {
                "x(ครึ่ง)".to_string()
            } else {
                format!("x{}", p.count)
            };
            text_pills_arr.push(format!("{} mg {}", p.mg, pill_count_text));
        }
    }

    let (container_classes, day_content_html) = if day_type == "stop" || day_dose < FLOAT_TOLERANCE
    {
        let classes = "bg-gray-100 border-gray-300";
        let content = "<div class=\"text-sm text-gray-800\">(0.0 mg)</div><div class=\"flex-grow flex flex-col justify-center items-center my-2 min-h-[30px]\"><div class=\"text-xs text-gray-500\">ไม่มีขนาดยา</div></div>".to_string();
        (classes, content)
    } else {
        let classes = if day_type == "special" {
            "bg-white border-red-400 border-2"
        } else {
            "bg-white border-gray-300"
        };
        let text_pills_html = text_pills_arr
            .iter()
            .map(|t| format!("<div class=\"text-xs text-gray-600\">{}</div>", t))
            .collect::<String>();
        let content = format!(
            "<div class=\"text-sm text-gray-800\">({:.1} mg)</div><div class=\"flex-grow flex flex-col justify-center items-center my-2 min-h-[30px]\"><div class=\"flex justify-center items-center flex-wrap\">{}</div></div><div>{}</div>",
            day_dose, if visual_pills.is_empty() { "&nbsp;" } else { &visual_pills }, text_pills_html
        );
        (classes, content)
    };

    format!(
        "<div class=\"rounded-lg border text-center flex flex-col h-full overflow-hidden {}\"> \
            <div class=\"font-bold text-gray-800 py-1 {}\">{}</div> \
            <div class=\"p-2 flex-grow flex flex-col\">{}</div> \
        </div>",
        container_classes, day_colors[idx], days_name[idx], day_content_html
    )
}

fn calculate_total_pills(
    option: &DosageOption,
    days_until_appointment: u32,
    start_day_of_week: u8,
) -> String {
    let mut half_pill_counts: HashMap<u8, u32> = HashMap::new();
    let mut whole_pill_counts: HashMap<u8, u32> = HashMap::new();

    for day in 0..days_until_appointment {
        let current_day_index = (start_day_of_week as u32 + day) % 7;
        let combo_for_day = match &option.option_type {
            OptionType::Uniform(c) => c,
            OptionType::NonUniform(cw) => &cw[current_day_index as usize],
        };
        for p in combo_for_day {
            if p.half {
                *half_pill_counts.entry(p.mg).or_insert(0) += p.count as u32;
            } else {
                *whole_pill_counts.entry(p.mg).or_insert(0) += p.count as u32;
            }
        }
    }

    let mut message = String::new();
    for &mg in &[5, 3, 2, 1] {
        let whole_count = *whole_pill_counts.get(&mg).unwrap_or(&0);
        let half_count = *half_pill_counts.get(&mg).unwrap_or(&0);
        let total_whole_pills_from_halves = half_count / 2;
        let remaining_halves = half_count % 2;

        let dispensed_pills = whole_count + total_whole_pills_from_halves + remaining_halves;
        if dispensed_pills > 0 {
            let actual_used = (whole_count + total_whole_pills_from_halves) as f64
                + (remaining_halves as f64 * 0.5);
            message.push_str(&format!(
                "<span class=\"pill pill-{}\"></span> {}mg: {} เม็ด",
                mg, mg, dispensed_pills
            ));
            if remaining_halves > 0 {
                message.push_str(&format!(" (ใช้จริง {:.1} เม็ด)", actual_used));
            }
            message.push_str("<br>");
        }
    }

    if message.is_empty() {
        "<span>ไม่ต้องจ่ายยา</span>".to_string()
    } else {
        message
    }
}

// --- Logic for getting day indices based on pattern ---

fn get_day_indices(
    num_stop: u8,
    num_special: u8,
    pattern: SpecialDayPattern,
) -> (Vec<usize>, Vec<usize>) {
    let mut stop = Vec::new();
    let mut special = Vec::new();
    match pattern {
        SpecialDayPattern::FriSun => {
            // Fri=4, Sat=5, Sun=6
            stop = (0..num_stop).map(|i| 6 - i as usize).collect();
            special = (0..num_special)
                .map(|i| 6 - num_stop as usize - i as usize)
                .collect();
        }
        SpecialDayPattern::MonWedFri => {
            // Mon=0, Wed=2, Fri=4
            if num_special == 3 {
                special = vec![0, 2, 4];
            } else if num_special == 2 {
                special = vec![0, 4];
                if num_stop == 1 {
                    stop = vec![2];
                }
            } else if num_special == 1 {
                if num_stop == 2 {
                    special = vec![2];
                    stop = vec![0, 4];
                } else if num_stop == 1 {
                    special = vec![0];
                    stop = vec![2];
                } else {
                    special = vec![2];
                }
            } else {
                // num_special == 0
                if num_stop == 3 {
                    stop = vec![0, 2, 4];
                } else if num_stop == 2 {
                    stop = vec![0, 4];
                } else if num_stop == 1 {
                    stop = vec![2];
                }
            }
        }
    }
    stop.sort();
    special.sort();
    (stop, special)
}

impl DosageOption {
    fn new(option_type: OptionType, weekly_dose_actual: f64) -> Self {
        let mut option = DosageOption {
            option_type,
            weekly_dose_actual,
            base_dose: 0.0,
            special_dose: 0.0,
            num_stop_days: 0,
            stop_days: Vec::new(),
            num_special_days: 0,
            special_days: Vec::new(),
            priority: 0,
            half_pill_complexity: 0,
            pill_color_count: 0,
            total_pill_objects: 0,
        };
        option.calculate_complexity();
        option
    }

    fn calculate_complexity(&mut self) {
        let mut half_pill_strengths = HashSet::new();
        let mut colors = HashSet::new();
        let mut total_pill_objs = 0;

        let combos_to_scan: Vec<Vec<Pill>> = match &self.option_type {
            OptionType::Uniform(c) => vec![c.clone(); 7],
            OptionType::NonUniform(cw) => cw.clone(),
        };

        for day_combo in combos_to_scan {
            for pill in day_combo {
                if pill.count > 0 {
                    colors.insert(pill.mg);
                    total_pill_objs += pill.count as u32;
                    if pill.half {
                        half_pill_strengths.insert(pill.mg);
                    }
                }
            }
        }

        self.priority = if let OptionType::Uniform(_) = self.option_type {
            0
        } else {
            1
        };
        self.half_pill_complexity = half_pill_strengths.len();
        self.pill_color_count = colors.len();
        self.total_pill_objects = total_pill_objs;
    }

    fn get_description(&self, days_name: &[&str]) -> String {
        match &self.option_type {
            OptionType::Uniform(combo) => {
                let daily_dose: f64 = combo
                    .iter()
                    .map(|p| p.mg as f64 * p.count as f64 * if p.half { 0.5 } else { 1.0 })
                    .sum();
                if daily_dose > FLOAT_TOLERANCE {
                    format!("ทุกวัน วันละ {:.1} mg", daily_dose)
                } else {
                    "หยุดยา".to_string()
                }
            }
            OptionType::NonUniform(_) => {
                let mut parts = Vec::new();
                if self.base_dose > FLOAT_TOLERANCE {
                    parts.push(format!("วันธรรมดา {:.1} mg", self.base_dose));
                }
                if self.num_special_days > 0 {
                    let special_day_names = self
                        .special_days
                        .iter()
                        .map(|&idx| days_name[idx])
                        .collect::<Vec<_>>()
                        .join(", ");
                    parts.push(format!(
                        "วันพิเศษ {:.1} mg ({})",
                        self.special_dose, special_day_names
                    ));
                }
                if self.num_stop_days > 0 {
                    let stop_day_names = self
                        .stop_days
                        .iter()
                        .map(|&idx| days_name[idx])
                        .collect::<Vec<_>>()
                        .join(", ");
                    parts.push(format!(
                        "หยุดยา {} วัน ({})",
                        self.num_stop_days, stop_day_names
                    ));
                }
                parts.join(", ")
            }
        }
    }
}
