use gettextrs::gettext;

pub const POWER_BUTTON_ACTIONS: [&str; 4] = ["Power Off", "Hibernate", "Suspend", "Nothing"];

pub const SUSPEND_DELAY_VALUES: [u32; 10] =
    [900, 1200, 1500, 1800, 2700, 3600, 4800, 5400, 6000, 7200];

pub const SCREEN_BLANK_DELAY_VALUES: [u32; 9] = [60, 120, 180, 240, 300, 480, 600, 720, 900];
