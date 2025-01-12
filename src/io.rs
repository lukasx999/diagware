pub mod eeprom;
pub mod dds;
pub mod shift_reg;
pub mod adc;



#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("I2C operation failed")]
    I2cError(#[from] rppal::i2c::Error),
    #[error("SPI operation failed")]
    SpiError(#[from] rppal::spi::Error),
    #[error("UTF8 operation failed")]
    Utf8Error(#[from] std::str::Utf8Error),
}

pub type IoResult<T> = Result<T, IoError>;
