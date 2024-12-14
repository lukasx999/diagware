use rppal::spi::{self, Spi, Bus, SlaveSelect, Mode};
use crate::db::model::Matrix;



#[derive(Debug)]
pub struct ShiftRegister {
    #[cfg(target_arch = "aarch64")]
    spi: Spi,
}

impl ShiftRegister {

    #[cfg(target_arch ="aarch64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {
            // TODO: handle multiple SPI devices on different buses
            spi: Spi::new(
                Bus::Spi0,
                SlaveSelect::Ss0,
                10e6,
                Mode::Mode0,
            ),
        })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {})
    }

    #[cfg(target_arch ="aarch64")]
    pub fn switch(&mut self, matrix: &Matrix) -> spi::Result<()> {



        Ok(())
    }

}

