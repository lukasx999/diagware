use rppal::spi::{self, Spi};



#[derive(Debug)]
pub struct DDS {
    #[cfg(target_arch ="aarch64")]
    spi: Spi,
}

impl DDS {

    #[cfg(target_arch ="aarch64")]
    pub fn new() -> i2c::Result<Self> {
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {})
    }

}

