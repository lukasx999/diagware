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

mod error;
use error::AnyError;





fn main() -> AnyError<()> {



    let db = DB::new()?;

    // let eeprom = EEPROM::new()?;
    // let diagnosis = Diagnosis::new(eeprom, db);

    // TODO: set up state machine UI + Visio Diagram + Infrastructure
    ui::run_gui(db)?;


    Ok(())

}
