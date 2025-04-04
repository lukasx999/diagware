mod ui;
mod db;
mod diagnosis;
mod logger;
mod io;
// pub for re-exporting in components prelude
pub mod config;
pub mod util;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        // spawned child processes like mount() should run as root
        libc::setuid(0);
    }

    ui::run_gui()?;
    Ok(())
}
