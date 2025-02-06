use std::error::Error;

mod ui;

mod db;
use db::DB;

mod diagnosis;
use diagnosis::Diagnosis;

mod io;
use io::{
    eeprom::EEPROM,
    dds::DDS,
    shift_reg::ShiftRegister,
    adc::ADC,
};

mod transfer;
use transfer::Transfer;

mod logger;
use logger::Logger;

pub mod util;


fn mount() {

    let mountdir = "/mnt/usb";
    let device   = "/dev/sda1";

    let devices = drives::get_devices().unwrap();
    for dev in devices {
        dbg!(dev);
    }

    /*
    let status: Option<i32> = std::process::Command::new("mount")
        .args([device, mountdir])
        .status()
        .expect("failed to execute process")
        .code();
    dbg!(status);


    let status: Option<i32> = std::process::Command::new("umount")
        .arg(mountdir)
        .status()
        .expect("failed to execute process")
        .code();
    dbg!(status);
    */

}



fn main() -> Result<(), Box<dyn Error>> {

    //unsafe {
    //    let mut err = libc::setuid(0);
    //    assert_eq!(err, 0);
    //    err = libc::seteuid(1000); // prevent us from accidentely messing something up as root
    //    assert_eq!(err, 0);
    //}

    //mount();

    ui::run_gui()?;

    Ok(())

}
