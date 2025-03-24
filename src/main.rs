mod ui;
mod db;
mod diagnosis;
mod logger;
mod io;
// pub for re-exporting in components prelude
pub mod config;
pub mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    ui::run_gui()?;

    Ok(())

}
