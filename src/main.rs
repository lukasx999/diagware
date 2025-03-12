mod ui;

mod db;
use db::DB;

mod diagnosis;

mod io;
use io::{
    eeprom::EEPROM,
    dds::DDS,
    shift_reg::ShiftRegister,
    adc::ADC,
};

mod logger;
use logger::Logger;

pub mod util;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    unsafe {
        let err = libc::setuid(0);
        assert_eq!(err, 0);
        let err = libc::seteuid(1000); // prevent us from accidentely messing something up as root
        assert_eq!(err, 0);
    }

    ui::run_gui()?;

    Ok(())

}
