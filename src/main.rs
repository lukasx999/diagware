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
        let err = libc::setuid(0);
        assert_eq!(err, 0);
        let err = libc::seteuid(1000); // prevent us from accidentely messing something up as root
        assert_eq!(err, 0);
    }

    ui::run_gui()?;

    Ok(())

}
