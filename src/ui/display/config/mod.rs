pub use cc_display_config::*;
pub use cc_display_mode::*;
pub use cc_display_monitor::*;
pub use cc_logical_monitor::*;
pub use types::*;

mod cc_display_config;
mod cc_display_mode;
mod cc_display_monitor;
mod cc_logical_monitor;
mod types;

pub trait GetList<T> {
    fn get_list(&self) -> Vec<T>;
}

pub trait GetListVia<T, V> {
    fn get_list_via(&self, via: &V) -> Vec<T>;
}
