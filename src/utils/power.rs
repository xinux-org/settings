pub const POWER_BUTTON_ACTIONS: [&str; 4] = ["Power Off", "Hibernate", "Suspend", "Nothing"];

pub const SUSPEND_DELAY_VALUES: [u32; 10] =
    [900, 1200, 1500, 1800, 2700, 3600, 4800, 5400, 6000, 7200];

pub const SUSPEND_DELAY_LABELS: [&str; 10] = [
    "15 minute",
    "20 minute",
    "25 minute",
    "30 minute",
    "45 minute",
    "1 hour",
    "1 hour 20 minute",
    "1 hour 30 minute",
    "1 hour 40 minute",
    "2 hours",
];

pub const SCREEN_BLANK_DELAY_VALUES: [u32; 9] = [60, 120, 180, 240, 300, 480, 600, 720, 900];
pub const SCREEN_BLANK_DELAY_LABELS: [&str; 9] = [
    "1 minute",
    "2 minutes",
    "3 minutes",
    "4 minutes",
    "5 minutes",
    "8 minutes",
    "10 minutes",
    "12 minutes",
    "15 minutes",
];
