mod ui;
mod db;
mod diagnosis;
mod logger;
mod io;
// pub for re-exporting in components prelude
pub mod config;
pub mod util;

const LOGFILE_DEV: &str = "diaglog";

// Redirect stdout to logfile
fn setup_logging() -> Result<*mut libc::FILE, std::ffi::NulError> {
    use std::ffi::CString;
    use std::ptr::null_mut;

    unsafe {
        let path = CString::new(format!("{}/{LOGFILE_DEV}", env!("HOME")))?;
        let mode = CString::new("w")?;
        let file = libc::fopen(path.as_ptr(), mode.as_ptr());
        assert_ne!(file, null_mut());

        libc::dup2(libc::fileno(file), libc::STDOUT_FILENO);
        Ok(file)
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        assert_eq!(libc::setuid(0), 0, "Setting uid failed. Are you running as root?");
        // prevent us from accidentely messing something up as root
        assert_eq!(libc::seteuid(1000), 0);
    }


    let file = setup_logging()?;

    println!("foobar");

    ui::run_gui()?;

    unsafe { libc::fclose(file); }
    Ok(())

}
