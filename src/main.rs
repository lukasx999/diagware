mod ui;

mod io;
use io::EEPROM;

mod db;
use db::{
    DB,
    model::{Module, TargetValue}
};

mod diagnosis;
use diagnosis::Diagnosis;




type AnyError<T> = Result<T, Box<dyn std::error::Error>>;








fn main() -> AnyError<()> {


    // let eeprom = EEPROM::new()?;
    //
    // let serial: String = eeprom.get_serial()?;
    // let module: Module = db.get_module_by_serial(serial.as_str()).await?;
    // dbg!(module);


    let db = DB::new()?;

    ui::run_gui(db)?;







    // dbg!(db.get_targetvalues_all().await?);


    // db.get_modules_all().await?;
    // db.module_add(Module::new(None, "esp32", "123")).await?;
    // db.module_delete_by_id(4).await?;






    Ok(())

}
