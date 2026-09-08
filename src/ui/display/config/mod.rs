pub use cc_display_config::*;
pub use cc_display_mode::*;
pub use cc_display_monitor::*;
pub use cc_logical_monitor::*;

mod cc_display_config;
mod cc_display_mode;
mod cc_display_monitor;
mod cc_logical_monitor;

pub trait GetList<T> {
    fn get_list(&self) -> Vec<T>;
}
