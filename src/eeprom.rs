use rppal::i2c::{self, I2c};

use std::time::Duration;
use std::thread;
use std::error::Error;


// Show I2C devices:
// `i2cdetect -y 1`
// Dump contents of I2C device:
// `i2cdump 1 0x50`


// Modify here!
const EEPROM_ADDRESS: u16 = 0x50;
const EEPROM_I2C_BUS: u8  = 0x1;

// Do not modify
const EEPROM_COLUMNS:      usize = 16;
const EEPROM_ROW_MAX:      u8    = 0xf0;
const EEPROM_ROW_STEP:     usize = 16;
const EEPROM_I2C_DELAY_MS: u64   = 10; // Delay after I2C transmission
const EEPROM_CLEAR_BYTE:   u8    = 0x0; // MUST be 0 for null termination

// NOTE: `command` is actually the row of the eeprom memory
// NOTE: delay is required after I2C operation (?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????)








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

    fn delay() {
        thread::sleep(Duration::from_millis(EEPROM_I2C_DELAY_MS));
    }


    #[cfg(target_arch ="aarch64")]
    pub fn get_serial(&self) -> Result<String, Box<dyn Error>> {

        let mut buf = [0_u8; EEPROM_COLUMNS];
        self.i2c.block_read(0x0, &mut buf)?;
        Self::delay();

        let s: String = std::str::from_utf8(&buf)?
            .trim_matches(char::from(0)) // strip nullbytes
            .to_owned();

        Ok(s)

    }

    #[cfg(target_arch = "x86_64")]
    pub fn get_serial(&self) -> Result<String, Box<dyn Error>> {
        Ok("123".to_owned())
    }


    // Accepts Strings with max. `EEPROM_COLUMNS` (=16) characters
    pub fn write_serial(&self, serial: &str) -> i2c::Result<()> {
        // TODO: return eeprom error
        assert!(serial.len() <= EEPROM_COLUMNS);

        self.clear()?;
        let bytes: &[u8] = serial.as_bytes();

        self.i2c.block_write(0x0, bytes)?;
        Self::delay();

        Ok(())
    }

    pub fn clear(&self) -> i2c::Result<()> {
        let bytes = [EEPROM_CLEAR_BYTE; EEPROM_COLUMNS];

        for row in (0x0..=EEPROM_ROW_MAX).step_by(EEPROM_ROW_STEP) {
            self.i2c.block_write(row, &bytes)?;
            Self::delay();
        }

        Ok(())
    }

}

