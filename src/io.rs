use rppal::{
    gpio::Gpio,
    i2c::{self, I2c},
    pwm::{Channel, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
    uart::{Parity, Uart},
};


// Show I2C devices:
// `i2cdetect -y 1`
// Dump contents of I2C device:
// `i2cdump 1 0x50`


// Modify here!
const EEPROM_ADDRESS: u16 = 0x50;
const EEPROM_I2C_BUS: u8  = 0x1;

// Do not modify
const EEPROM_COLUMNS:  usize = 16;
const EEPROM_ROW_MAX:  u8    = 0xf0;
const EEPROM_ROW_STEP: usize = 16;


type AnyError<T> = Result<T, Box<dyn std::error::Error>>;





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


    // NOTE: `command` is actually the row of the eeprom memory

    // BUG: crash when reading after writing
    // BUG: crash when writing after writing


    // #[cfg(target_arch = "x86_64")]
    // #[cfg(target_arch ="aarch64")]

    pub fn get_serial_int(&self) -> i2c::Result<i32> {

        let mut buf = [0_u8; 4];
        self.i2c.block_read(0x0, &mut buf)?;
        Ok(i32::from_ne_bytes(buf))

    }

    pub fn write_serial_int(&self, serial: i32) -> i2c::Result<()> {

        let bytes: [u8; 4] = serial.to_ne_bytes();
        self.i2c.block_write(0x0, &bytes)?;
        Ok(())

    }

    pub fn get_serial(&self) -> AnyError<String> {

        let mut buf = [0_u8; 4]; // TODO: change byte amount: encoding string length or nullbyte
        self.i2c.block_read(0x0, &mut buf)?;
        Ok(String::from_utf8(buf.to_vec())?)

    }

    // Accepts Strings with less than `EEPROM_COLUMNS` (=16) characters
    pub fn write_serial(&self, serial: &str) -> i2c::Result<()> {

        assert!(serial.len() <= EEPROM_COLUMNS);

        // self.clear();

        let bytes: &[u8] = serial.as_bytes();
        self.i2c.block_write(0x0, bytes)?;
        Ok(())
    }

    pub fn clear(&self) -> i2c::Result<()> {
        let bytes = [0_u8; EEPROM_COLUMNS];

        // BUG: not working
        for row in (0x0..=EEPROM_ROW_MAX).step_by(EEPROM_ROW_STEP) {
            self.i2c.block_write(row, &bytes)?;
        }

        Ok(())
    }

}

