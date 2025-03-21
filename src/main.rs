mod ui;
mod db;
mod diagnosis;
mod logger;
mod io;
// pub for re-exporting in components prelude
pub mod config;
pub mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //unsafe {
    //    assert_eq!(libc::setuid(0), 0, "Setting uid failed. Are you running as root?");
    //    // prevent us from accidentely messing something up as root
    //    assert_eq!(libc::seteuid(1000), 0);
    //}

    ui::run_gui()?;

    Ok(())

}
