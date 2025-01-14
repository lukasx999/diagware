
pub struct Xfer {
}


impl Xfer {

    pub fn new() -> Self {
        Self {}
    }

    fn mount() {
        let status: Option<i32> = std::process::Command::new("mount")
            .args(["/dev/sda1", "/home/lukas/usb"])
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);
    }

    fn unmount() {
        let status: Option<i32> = std::process::Command::new("umount")
            .args(["/home/lukas/usb"])
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);
    }

}
