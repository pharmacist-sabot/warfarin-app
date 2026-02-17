export type PillType = {
  mg: number;
  colorClass: string;
};

export type AvailablePills = Record<number, boolean>;

export type AppointmentInfo = {
  valid: boolean;
  daysUntilAppointment: number;
  startDayOfWeek: number;
};

export type CalculationInput = {
  weekly_dose: number;
  allow_half: boolean;
  available_pills: number[];
  special_day_pattern: string;
  days_until_appointment: number;
  start_day_of_week: number;
};

export type PillRenderData = {
  mg: number;
  count: number;
  is_half: boolean;
};

export type DaySchedule = {
  day_index: number;
  total_dose: number;
  pills: PillRenderData[];
  is_stop_day: boolean;
  is_special_day: boolean;
};

export type PillLineSummary = {
  mg: number;
  dispensed_count: number;
  usage_note: string;
};

export type TotalPillsSummary = {
  header: string;
  pill_lines: PillLineSummary[];
};

export type RegimenOption = {
  description: string;
  weekly_dose_actual: number;
  weekly_schedule: DaySchedule[];
  total_pills_summary: TotalPillsSummary;
};
