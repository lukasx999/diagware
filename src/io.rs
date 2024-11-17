use rppal::{
    gpio::Gpio,
    i2c::{self, I2c},
    pwm::{Channel, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
    uart::{Parity, Uart},
};


// Modify here!
const EEPROM_ADDRESS: u16 = 0x50;
const EEPROM_I2C_BUS: u8  = 0x1;



#[derive(Debug)]
pub struct EEPROM {
    i2c: I2c,
}

impl EEPROM {

    pub fn new() -> i2c::Result<Self> {
        let mut i2c = I2c::with_bus(EEPROM_I2C_BUS)?;
        i2c.set_slave_address(EEPROM_ADDRESS)?;
        Ok(Self { i2c })
    }

    pub fn read(&mut self) -> i2c::Result<()> {
        let mut buf = [0_u8; 5];
        self.i2c.read(&mut buf)?;
        dbg!(&buf);
        Ok(())
    }

}

