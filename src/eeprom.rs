use rppal::i2c::{self, I2c};

use std::time::Duration;


// Show I2C devices:
// `i2cdetect -y 1`
// Dump contents of I2C device:
// `i2cdump 1 0x50`


// TODO: create config.rs
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



// TODO: consider using the `thiserror` crate

#[derive(Debug)]
pub enum EepromError {
    I2cError(i2c::Error),
    Utf8Error(std::str::Utf8Error),
}

impl std::fmt::Display for EepromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::I2cError(_) => "I2C operation failed",
            Self::Utf8Error(_) => "UTF8 operation failed",
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for EepromError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<i2c::Error> for EepromError {
    fn from(value: i2c::Error) -> Self {
        Self::I2cError(value)
    }
}

impl From<std::str::Utf8Error> for EepromError {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8Error(value)
    }
}

pub type EepromResult<T> = std::result::Result<T, EepromError>;





#[derive(Debug)]
pub struct EEPROM {
    #[cfg(target_arch = "aarch64")]
    i2c: I2c,
}

impl EEPROM {

    #[cfg(target_arch = "aarch64")]
    pub fn new() -> EepromResult<Self> {
        let mut i2c = I2c::with_bus(EEPROM_I2C_BUS)?;
        i2c.set_slave_address(EEPROM_ADDRESS)?;
        Ok(Self { i2c })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> EepromResult<Self> {
        Ok(Self {})
    }

    fn delay() {
        std::thread::sleep(Duration::from_millis(EEPROM_I2C_DELAY_MS));
    }


    #[cfg(target_arch ="aarch64")]
    pub fn get_serial(&self) -> EepromResult<String> {

        let mut buf = [0_u8; EEPROM_COLUMNS];
        self.i2c.block_read(0x0, &mut buf)?;
        Self::delay();

        let s: String = std::str::from_utf8(&buf)?
            .trim_matches(char::from(0)) // strip nullbytes
            .to_owned();

        Ok(s)

    }

    #[cfg(target_arch = "x86_64")]
    pub fn get_serial(&self) -> EepromResult<String> {
        Ok("123".to_owned())
    }


    // Accepts Strings with max. `EEPROM_COLUMNS` (=16) characters
    #[cfg(target_arch ="aarch64")]
    pub fn write_serial(&self, serial: &str) -> EepromResult<()> {
        // TODO: return eeprom error
        assert!(serial.len() <= EEPROM_COLUMNS);

        self.clear()?;
        let bytes: &[u8] = serial.as_bytes();

        self.i2c.block_write(0x0, bytes)?;
        Self::delay();

        Ok(())
    }

    #[cfg(target_arch ="aarch64")]
    pub fn clear(&self) -> EepromResult<()> {
        let bytes = [EEPROM_CLEAR_BYTE; EEPROM_COLUMNS];

        for row in (0x0..=EEPROM_ROW_MAX).step_by(EEPROM_ROW_STEP) {
            self.i2c.block_write(row, &bytes)?;
            Self::delay();
        }

        Ok(())
    }

}
