mod io;
use io::EEPROM;

mod db;
use db::{DB, Module};

type AnyError<T> = Result<T, Box<dyn std::error::Error>>;


#[tokio::main]
async fn main() -> AnyError<()> {

    println!("diagware!");

    // let db = DB::new().await?;
    // db.get_modules_all().await?;
    // db.module_add(Module::new(None, "esp32", "123")).await?;
    // db.module_delete_by_id(4).await?;


    let eeprom = EEPROM::new()?;
    eeprom.write_serial("greetings")?;
    // dbg!(eeprom.get_serial()?);
    // eeprom.clear()?;


    Ok(())

}
