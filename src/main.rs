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

    unsafe {
        let path = CString::new(format!("{}/{LOGFILE_DEV}", env!("HOME")))?;
        let mode = CString::new("w")?;
        let file = libc::fopen(path.as_ptr(), mode.as_ptr());

        libc::dup2(libc::fileno(file), libc::STDOUT_FILENO);
        Ok(file)
    }

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        use std::ffi::CString;
        let path = CString::new("/dev/nvme0")?;
        let mode = CString::new("w")?;
        let f = libc::fopen(path.as_ptr(), mode.as_ptr());
        dbg!(&f);
    }

    return Ok(());

    unsafe {
        assert_eq!(libc::setuid(0), 0);
        // prevent us from accidentely messing something up as root
        assert_eq!(libc::seteuid(1000), 0);
    }


    let file = setup_logging()?;

    println!("foobar");

    ui::run_gui()?;

    unsafe { libc::fclose(file); }
    Ok(())

}
