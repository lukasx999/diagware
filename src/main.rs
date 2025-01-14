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

    unsafe {
        libc::setuid(0);
        libc::seteuid(1000); // prevent us from accidentely messing up something as root
    }

    // let result = drives::get_devices();
    // for device in result.unwrap() {
    //     dbg!(device);
    // }


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
