
mod prelude {
    pub use std::sync::{Arc, Mutex};
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use std::collections::HashMap;
    pub use egui::Color32;
}

pub mod pinview;
pub mod serialmanager;
pub mod diagnosis;
pub mod dbmanager;
pub mod logging;
pub mod topbar;
pub mod documents;
