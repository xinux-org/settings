use crate::config::{APP_NAME, STATE_FILE};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Page {
    WiFi,
    // Network,
    // Bluetooth,
    Display,
    Appearance,
    // Sound,
    Power,
    // Multitasking,
    Apps,
    Notifications,
    // Search,
    // Accounts,
    // Sharing,
    // Wellbeing,
    Mouse,
    // Accesibility,
    // PrivacyAndSecurity,
    System,
}

impl Page {
    pub fn value(&self) -> String {
        match self {
            Self::WiFi => String::from("wifi"),
            // Self::Network => String::from("network"),
            // Self::Bluetooth => String::from("bluetooth"),
            Self::Display => String::from("display"),
            Self::Appearance => String::from("appearance"),
            // Self::Sound => String::from("sound"),
            Self::Power => String::from("power"),
            // Self::Multitasking => String::from("multitasking"),
            Self::Apps => String::from("apps"),
            Self::Notifications => String::from("notifications"),
            // Self::Search => String::from("search"),
            // Self::Accounts => String::from("accounts"),
            // Self::Sharing => String::from("sharing"),
            // Self::Wellbeing => String::from("wellbeing"),
            Self::Mouse => String::from("mouse"),
            // Self::Accesibility => String::from("accessibility"),
            // Self::PrivacyAndSecurity => String::from("privacyandsecurity"),
            Self::System => String::from("system"),
        }
    }
}

impl From<String> for Page {
    fn from(value: String) -> Self {
        match value.as_str() {
            "wifi" => Self::WiFi,
            // "network" => Some(Self::Network),
            // "bluetooth" => Some(Self::Bluetooth),
            "display" => Self::Display,
            "appearance" => Self::Appearance,
            // "sound" => Some(Self::Sound),
            "power" => Self::Power,
            // "multitasking" => Some(Self::Multitasking),
            "apps" => Self::Apps,
            "notifications" => Self::Notifications,
            // "search" => Some(Self::Search),
            // "accounts" => Some(Self::Accounts),
            // "sharing" => Some(Self::Sharing),
            // "wellbeing" => Some(Self::Wellbeing),
            "mouse" => Self::Mouse,
            // "accessibility" => Some(Self::Accesibility),
            // "privacyandsecurity" => Some(Self::PrivacyAndSecurity),
            "system" => Self::System,
            _ => Self::WiFi,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub page: Option<Page>,
}

pub fn get_state() -> Option<State> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME);
    let path = xdg_dirs.get_state_file(STATE_FILE)?;
    let raw_state = fs::read_to_string(path).ok()?;
    let state: State = toml::from_str(&raw_state).ok()?;

    Some(state)
}

pub fn save_state(state: State) -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME);
    let path = xdg_dirs
        .get_state_file(STATE_FILE)
        .ok_or_else(|| anyhow!("Can't get state file: {}", STATE_FILE))?;

    if !fs::exists(&path)? {
        xdg_dirs.place_state_file(STATE_FILE)?;
    }

    let raw_state = toml::to_string(&state)?;

    fs::write(path, raw_state)?;

    Ok(())
}

pub fn update_state<F>(f: F) -> Result<()>
where
    F: FnOnce(&mut State),
{
    let mut state = get_state().unwrap_or(State { page: None });

    f(&mut state);

    save_state(state)
}
