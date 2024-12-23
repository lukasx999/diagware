use rppal::spi::{self, Spi, Bus, SlaveSelect, Mode};
use crate::db::model::Matrix;


// TODO: handle multiple SPI devices on different buses



#[derive(Debug)]
pub struct ShiftRegister {
    #[cfg(target_arch = "aarch64")]
    spi: Spi,
}

impl ShiftRegister {

    #[cfg(target_arch ="aarch64")]
    pub fn new() -> spi::Result<Self> {

        let spi = Spi::new(
            Bus::Spi0,
            SlaveSelect::Ss0,
            10e6 as u32,
            Mode::Mode0,
        )?;

        Ok(Self { spi })
    }

    #[cfg(target_arch = "x86_64")]
    pub fn new() -> spi::Result<Self> {
        Ok(Self {})
    }

    fn int_to_bits(number: u16) -> Vec<bool> {

        let bit_count = size_of::<u16>() * 8;

        let bits: Vec<bool> = (0..bit_count).map(|i| {
            let a = (number >> i) & 1;
            a != 0
        }).collect();

        bits

    }



    #[cfg(target_arch = "aarch64")]
    pub fn switch(&mut self, matrix: &Matrix) -> spi::Result<()> {

        let bits = Self::int_to_bits(matrix.gnd);
        dbg!(bits);





        Ok(())
    }

}

