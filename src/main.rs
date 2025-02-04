use std::error::Error;

mod ui;

mod db;
use db::DB;

mod diagnosis;
use diagnosis::Diagnosis;

mod io;
use io::{
    eeprom::EEPROM,
    dds::DDS,
    shift_reg::ShiftRegister,
    adc::ADC,
};

mod transfer;
use transfer::Transfer;

mod logger;
use logger::Logger;

pub mod util;




fn main() -> Result<(), Box<dyn Error>> {

    #[cfg(not(debug_assertions))]
    unsafe {
        let mut err = libc::setuid(0);
        assert_eq!(err, 0);
        err = libc::seteuid(1000); // prevent us from accidentely messing up something as root
        assert_eq!(err, 0);
    }

    // unsafe {
    //     let err = libc::mount("/dev/sda", "~/usb");
    //     assert_eq!(err, 0);
    // }

    ui::run_gui()?;

    Ok(())

}
