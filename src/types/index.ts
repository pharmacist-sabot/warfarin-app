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

export type RegimenOption = {
  description: string;
  weekly_dose_actual: number;
  weekly_schedule_html: string;
  total_pills_message: string;
};
