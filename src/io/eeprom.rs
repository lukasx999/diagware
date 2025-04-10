#[cfg(target_arch = "aarch64")]
use std::time::Duration;

#[cfg(target_arch = "aarch64")]
use rppal::i2c::{self, I2c};

use crate::io::IoResult;

pub const EEPROM_SERIAL_MAX_SIZE: usize = 16;

//
// eeprom: AT24C02D
// Show I2C devices:
// `i2cdetect -y 1`
// Dump contents of I2C device:
// `i2cdump 1 0x50`
//

#[cfg(target_arch = "aarch64")] const EEPROM_ADDRESS: u16 = 0x50;
#[cfg(target_arch = "aarch64")] const EEPROM_I2C_BUS: u8  = 0x1;

// Do not modify
#[cfg(target_arch = "aarch64")] const EEPROM_COLUMNS:      usize = 16;
#[cfg(target_arch = "aarch64")] const EEPROM_ROW_MAX:      u8    = 0xf0;
#[cfg(target_arch = "aarch64")] const EEPROM_ROW_STEP:     usize = 16;
#[cfg(target_arch = "aarch64")] const EEPROM_I2C_DELAY_MS: u64   = 10; // Delay after I2C transmission



#[derive(Debug)]
pub struct EEPROM {
    #[cfg(target_arch = "aarch64")] i2c: I2c,
}

impl EEPROM {

    #[cfg(target_arch = "aarch64")]
    pub fn new() -> IoResult<Self> {
        let mut i2c = I2c::with_bus(EEPROM_I2C_BUS)?;
        i2c.set_slave_address(EEPROM_ADDRESS)?;
        Ok(Self { i2c })
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn new() -> IoResult<Self> {
        Ok(Self {})
    }

    #[cfg(target_arch = "aarch64")]
    fn delay() {
        std::thread::sleep(Duration::from_millis(EEPROM_I2C_DELAY_MS));
    }

    #[cfg(target_arch = "aarch64")]
    pub fn get_serial(&self) -> IoResult<String> {

        let mut buf = [0; EEPROM_COLUMNS];
        self.i2c.block_read(0x0, &mut buf)?;
        Self::delay();

        let s = std::str::from_utf8(&buf)?
            .trim_matches(char::from(0)) // strip nullbytes
            .to_owned();

        Ok(s)
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn get_serial(&self) -> IoResult<String> {
        Ok("214232".to_owned())
    }

    // strings may be no longer than `EEPROM_SERIAL_MAX_SIZE` bytes
    #[cfg(target_arch = "aarch64")]
    pub fn write_serial(&self, serial: &str) -> IoResult<()> {
        assert!(serial.len() <= EEPROM_SERIAL_MAX_SIZE);

        self.clear()?;
        let bytes: &[u8] = serial.as_bytes();

        self.i2c.block_write(0x0, bytes)?;
        Self::delay();

        Ok(())
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn write_serial(&self, _serial: &str) -> IoResult<()> {
        Ok(())
    }

    #[cfg(target_arch = "aarch64")]
    pub fn clear(&self) -> IoResult<()> {

        let bytes = [0x0; EEPROM_COLUMNS];

        for row in (0x0..=EEPROM_ROW_MAX).step_by(EEPROM_ROW_STEP) {
            self.i2c.block_write(row, &bytes)?;
            // delay is required after I2C operation (?)
            Self::delay();
        }

        Ok(())
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn clear(&self) -> IoResult<()> {
        Ok(())
    }

}
