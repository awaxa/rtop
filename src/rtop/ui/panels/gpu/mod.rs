mod processes;
mod memoryusage;
mod temps;
mod powerusage;

pub use self::processes::processes_panel as processes_panel;
pub use self::memoryusage::mem_history_panel as mem_history_panel;
pub use self::temps::temp_history_panel as temp_history_panel;
pub use self::powerusage::power_history_panel as power_history_panel;
