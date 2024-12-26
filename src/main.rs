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
};




fn main() -> Result<(), Box<dyn Error>> {

    let db       = DB::new()?;
    let eeprom   = EEPROM::new()?;
    let shiftreg = ShiftRegister::new()?;

    let (tx, rx) = std::sync::mpsc::channel(); // Channel for communication between diagnosis and gui

    let diagnosis = Diagnosis::new(
        eeprom,
        db,
        shiftreg,
        tx
    );

    ui::run_gui(diagnosis, rx)?;

    Ok(())

}
