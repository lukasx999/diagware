mod io;
use io::EEPROM;

mod db;
use db::{DB, Module};

type AnyError<T> = Result<T, Box<dyn std::error::Error>>;


use rppal::i2c::I2c;


#[tokio::main]
async fn main() -> AnyError<()> {

    println!("diagware!");

    // let db = DB::new().await?;
    // db.get_modules_all().await?;
    // db.module_add(Module::new(None, "esp32", "123")).await?;
    // db.module_delete_by_id(4).await?;


    // NOTE:
    // NOTE: delay is required after I2C operation
    // std::thread::sleep(std::time::Duration::new(1, 0));
    // NOTE:


    let mut i2c = I2c::with_bus(0x1)?;
    i2c.set_slave_address(0x50)?;
    let buf = [0_u8; 5];
    i2c.block_write(0, &buf)?;
    std::thread::sleep(std::time::Duration::new(1, 0));
    i2c.block_write(0, &buf)?;


    // let eeprom = EEPROM::new()?;
    // eeprom.write_serial("greetings")?;
    // eeprom.write_new()?;
    // dbg!(eeprom.get_serial()?);
    // eeprom.clear()?;


    Ok(())

}
