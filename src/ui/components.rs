mod prelude {
    pub use std::sync::{Arc, Mutex};
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use crate::logger::{Logger, LogLevel};
    pub use crate::ui::{Component, config};
    pub use crate::util;
    pub use egui::Color32;
}

pub mod serialmanager;
pub mod diagnosis;
pub mod logging;
pub mod topbar;
pub mod documents;
