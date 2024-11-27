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
    let diagnosis = Diagnosis::new();
    // let eeprom = EEPROM::new()?;

    ui::run_gui(db, diagnosis)?;


    Ok(())

}
