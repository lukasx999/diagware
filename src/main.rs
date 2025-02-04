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

    // let xfer = Transfer::new("/home/lukas/usb".to_owned());


    let logger   = Logger::new();
    let db       = DB::new()?;
    let eeprom   = EEPROM::new()?;
    let dds      = DDS::new()?;
    let shiftreg = ShiftRegister::new()?;
    let adc      = ADC::new()?;

    let (tx, rx) = std::sync::mpsc::channel(); // Channel for communication between diagnosis and gui

    let diagnosis = Diagnosis::new(
        eeprom,
        db,
        shiftreg,
        dds,
        adc,
        tx
    );

    ui::run_gui(diagnosis, rx, logger)?;

    Ok(())

}
