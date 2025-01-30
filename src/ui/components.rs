mod prelude {
    pub use std::sync::{Arc, Mutex};
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use egui::Color32;
    pub use crate::logger::{Logger, LogLevel};
    pub use crate::ui::{Component, config};
    pub use crate::diagnosis::Diagnosis;
    pub use crate::util;
}

pub mod pinview;
pub mod serialmanager;
pub mod diagnosis;
pub mod logging;
pub mod topbar;
pub mod documents;
