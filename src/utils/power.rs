/// Possible actions for power button(usually turn on/off)
/// - Power Off - which is sometimes stated as Interactive will prompt you to decide if you really to turn off your device.
/// - Hibernate - turns of the device after copying the current state of running applications from RAM to SWAP(if configured)
/// - Suspend - does NOT turn off the device, instead, it switches to sleep mode or low power consumption mode keeping the applications open and running.
/// - Nothing - the name is self explanatory.
pub const POWER_BUTTON_ACTIONS: [&str; 4] = ["Power Off", "Hibernate", "Suspend", "Nothing"];

pub const SUSPEND_DELAY_VALUES: [u32; 10] =
    [900, 1200, 1500, 1800, 2700, 3600, 4800, 5400, 6000, 7200];

pub const SCREEN_BLANK_DELAY_VALUES: [u32; 9] = [60, 120, 180, 240, 300, 480, 600, 720, 900];
