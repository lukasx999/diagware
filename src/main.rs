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

mod xfer;
use xfer::Xfer;



fn main() -> Result<(), Box<dyn Error>> {

    // TODO: shutdown

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

    // let xfer = Xfer::new("/home/lukas/usb".to_owned());



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

    ui::run_gui(diagnosis, rx)?;

    Ok(())

}
