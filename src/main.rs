use std::error::Error;

mod ui;

mod eeprom;
use eeprom::EEPROM;

mod dds;
use dds::DDS;

mod db;
use db::DB;

mod diagnosis;
use diagnosis::Diagnosis;





fn main() -> Result<(), Box<dyn Error>> {

    let db     = DB::new()?;
    let eeprom = EEPROM::new()?;

    let diagnosis = Diagnosis::new(
        eeprom,
        db,
    );

    ui::run_gui(diagnosis)?;



    Ok(())

}
