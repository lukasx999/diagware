use std::error::Error;

mod ui;

mod eeprom;
use eeprom::EEPROM;

mod dds;
use dds::DDS;

mod db;
use db::DB;

mod shift_reg;
use shift_reg::ShiftRegister;

mod diagnosis;
use diagnosis::Diagnosis;



fn main() -> Result<(), Box<dyn Error>> {

    let db     = DB::new()?;
    let eeprom = EEPROM::new()?;

    let (tx, rx) = std::sync::mpsc::channel(); // Channel for communication between diagnosis and gui

    let diagnosis = Diagnosis::new(
        eeprom,
        db,
        tx
    );

    ui::run_gui(diagnosis, rx)?;


    Ok(())

}
