#[cfg(target_arch = "aarch64")]
use rppal::spi::{self, Spi, Bus, SlaveSelect, Mode};

use crate::io::IoResult;

pub type MeasurementData = f64;

#[derive(Debug)]
pub struct ADC {
    #[cfg(target_arch = "aarch64")] spi: Spi,
}

impl ADC {

    #[cfg(target_arch = "aarch64")]
    pub fn new() -> IoResult<Self> {

        let spi = Spi::new(
            Bus::Spi0,
            SlaveSelect::Ss0,
            10e6 as u32,
            Mode::Mode0,
        )?;

        Ok(Self { spi })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> IoResult<Self> {
        Ok(Self {})
    }

    #[cfg(target_arch = "x86_64")]
    pub fn measure(&mut self) -> IoResult<MeasurementData> {
        Ok(45.0)
    }

    #[cfg(target_arch = "aarch64")]
    pub fn measure(&mut self) -> IoResult<MeasurementData> {
        // TODO:
        Ok(0.0)
    }

}

