pub struct Transfer {
    mountdir: String,
}

// TODO: umount() when mem::drop()'d


#[allow(dead_code)]
impl Transfer {

    pub fn new(mountdir: String) -> Self {
        let s = Self {
            mountdir,
        };
        s.mount("/dev/sda1");
        s
    }

    fn get_devices() {
        todo!();
        // let result = drives::get_devices();
        // for device in result.unwrap() {
        //     dbg!(device);
        // }
    }

    fn mount(&self, device: &str) {
        let status: Option<i32> = std::process::Command::new("mount")
            .args([device, &self.mountdir])
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);
    }

    fn save_document(&self) {
        todo!();
    }

    fn unmount(&self) {
        let status: Option<i32> = std::process::Command::new("umount")
            .arg(&self.mountdir)
            .status()
            .expect("failed to execute process")
            .code();
        dbg!(status);
    }

}
