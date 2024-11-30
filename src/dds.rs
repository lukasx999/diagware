use rppal::spi::{self, Spi};



#[derive(Debug)]
pub struct DDS {
}

impl DDS {

    #[cfg(target_arch ="aarch64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {})
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {})
    }

}

