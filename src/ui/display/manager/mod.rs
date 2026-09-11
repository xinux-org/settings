pub use display_config::*;
pub use display_config_manager::*;
pub use display_config_proxy::*;
pub use display_mode::*;
pub use display_monitor::*;
pub use logical_monitor::*;
pub use types::*;

mod display_config;
mod display_config_manager;
mod display_config_proxy;
mod display_mode;
mod display_monitor;
mod logical_monitor;
mod types;

pub trait GetList<T> {
    fn get_list(&self) -> Vec<T>;
}

pub trait GetListVia<T, V> {
    fn get_list_via(&self, via: &V) -> Vec<T>;
}
