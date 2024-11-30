use std::{
    rc::Rc,
    sync::{Arc, Mutex},
    error::Error,
};

mod ui;

mod eeprom;
use eeprom::EEPROM;

mod db;
use db::{
    DB,
    model::{Module, TargetValue}
};

mod diagnosis;
use diagnosis::Diagnosis;





fn main() -> Result<(), Box<dyn Error>> {

    let db     = Arc::new(Mutex::new(DB::new()?));
    let eeprom = Arc::new(Mutex::new(EEPROM::new()?));

    let diagnosis = Diagnosis::new(
        eeprom.clone(),
        db.clone()
    );

    ui::run_gui(db, eeprom, diagnosis)?;



    Ok(())

}
