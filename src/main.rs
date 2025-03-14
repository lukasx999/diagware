mod ui;
mod db;
mod diagnosis;
mod logger;
mod io;
// pub for re-exporting in components prelude
pub mod config;
pub mod util;

//const LOGFILE_DEV: &str = "diaglog";
//
//// Redirect stdout to logfile
//#[derive(Debug, Clone)]
//struct LoggingContext(*mut libc::FILE);
//
//impl LoggingContext {
//    pub fn new() -> Result<Self, std::ffi::NulError> {
//        use std::ffi::CString;
//        use std::ptr::null_mut;
//
//        let file = unsafe {
//            let path = CString::new(format!("{}/{LOGFILE_DEV}", env!("HOME")))?;
//            let mode = CString::new("w")?;
//            let file = libc::fopen(path.as_ptr(), mode.as_ptr());
//            assert_ne!(file, null_mut());
//
//            libc::dup2(libc::fileno(file), libc::STDOUT_FILENO);
//            file
//        };
//
//        Ok(Self(file))
//    }
//}
//
//impl Drop for LoggingContext {
//    fn drop(&mut self) {
//        unsafe {
//            libc::fclose(self.0);
//        }
//    }
//}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        assert_eq!(libc::setuid(0), 0, "Setting uid failed. Are you running as root?");
        // prevent us from accidentely messing something up as root
        assert_eq!(libc::seteuid(1000), 0);
    }


    // Drop implementation will automatically close logfile at end of main
    //let _log = LoggingContext::new()?;

    ui::run_gui()?;

    Ok(())

}
