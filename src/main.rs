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



// TODO: change rust-analyzer target arch
// TODO: diagnosis logger in UI


fn main() -> Result<(), Box<dyn Error>> {

    let db     = DB::new()?;
    let eeprom = EEPROM::new()?;

    let t = db.get_targetvalues_all()?;
    dbg!(t);


    let (tx, rx) = std::sync::mpsc::channel(); // Channel for communication between diagnosis and gui

    let diagnosis = Diagnosis::new(
        eeprom,
        db,
        tx
    );

    ui::run_gui(diagnosis, rx)?;


    Ok(())

}
