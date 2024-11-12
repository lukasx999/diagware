use rppal::{
    gpio::Gpio,
    i2c::I2c,
    pwm::{Channel, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
    uart::{Parity, Uart},
};


pub struct EEPROM {
    i2c: I2c,
}




impl EEPROM {
    pub fn new() -> Self {

        let mut i2c = I2c::new().unwrap();
        i2c.set_slave_address(0x51);

        Self {
            i2c,
        }
    }
    pub fn write(&mut self) {
        let buf: &[u8] = "123".as_bytes();
        self.i2c.write(buf);
    }
}

