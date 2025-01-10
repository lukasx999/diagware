# Diagware

## Notes

- Housekeeping is 2nd priority

## Questions

- implementing usb-stick download is complicated, maybe serving documents over http will suffice?
- Multiple breakpoints?


## TODO

- [ ] Scripts for inserting binary data (eg images) as blob into DB
- [ ] writing to USB (documents / logfile) (popup selector for device (from lsblk))
- [ ] DB Manager - View DB
- [ ] Using sqlitestudio for DB management instead
- [ ] crosscompile to aarch64 -> rsync binary only
- [ ] egui: slider and preview for DDS waveform
- [ ] EGUI: Error Popup for error handling
- [ ] Multiple DB (stable/experimental)
- [ ] Hardware checker page to see if all devices are connected and working
- [ ] X11 isolated enviroment
- [ ] exercise/data sheets in DB
- [ ] File path handling for SQLx and egui::include\_image!
- [ ] Visio Drawing & Explanation for Drawing States
- [x] Set breakpoints for states
- [x] make `next` move to next state and execute that state
- [x] cyclic state loop
- [x] Diagnosis breakpoints
- [x] export new json to file (logger)
- [x] State machine for diagnosis
- [x] Cargo Script for uploading to rpi
- [x] Login
- [x] UI (probably last)
- [x] make use of async
- [x] Testing Code on RPi4
- [x] Writing serial to EEPROM


## Docs
- ImGui vs Retained-mode GUI
- State Machine diagram + explaination
- State Machine GUI rendering
- State and Ownership diagram (channels, arcs, mutexes, ...)
- R/W EEPROM
- DB Management (table & column layout)



## NOTES

- Egui ONLY redraws ui when moving cursor => `ctx.request_repaint()`
- Always (cargo) run from the project root
- Ensure that `.cargo/config.toml` holds the correct url to the database
  - (needed for compile-time query checking by sqlx)
- **RPi**: username: `pi`, password: `piuser4`
- Software
  - Sqlitestudio
  - `bacon clippy`
- Add rustup target for ARM architecutre
  - config in `.cargo/config.toml`
  - `rustup target add aarch64-unknown-linux-gnu`



## Procedures


### Login into RPi with SSH

- `ssh <user>@<ip>`


### Setting up Public Key authorization *(optional)*

On the host machine:

- `ssh-keygen -t rsa`
- `ssh-copy-id -i ~/.ssh/id_rsa.pub <user>@<ip>`


### Viewing I2C EEPROM on RPi

- `i2cdetect -y 1`
- `i2cdump 1 0x50`


### Uploading Code to RPi4

Run Upload Script (`up.sh`)\
**IMPORTANT**: Make sure the constants in `up.sh` are correct

- Upload and build
  - `./up.sh build` or just `./up.sh`
- Remotely run the code
  - `./up.sh run`



### Setting up DB

- Run setup script
  - `sqlite3 database.db < db_scripts/setup.sql` *(bash-specific)*\
  *or:*
  - `sqlite> .read db_scripts/setup.sql`

- Load Data *(optional)*
  - `db_scripts/examples.sql` *(for testing)*
  - `db_scripts/default.sql` *(for production)*
  - process is the same as setup

- Verify Setup
  - `sqlite> .tables`
  - `sqlite> .schema`
  - `sqlite> SELECT * FROM <table>`


### Installing Rust

- [Rustup](https://rustup.rs/) - run command specified for `Unix`


### Building

- `cargo build --release`
*or:*
- `cargo b -r`




## Dependencies

- **sqlx**
  - Database
- **egui**
  - UI
- **rppal**
  - Raspberry Pi4 IO (GPIO, I2C, SPI, ...)
- **tokio**
  - Rust Async runtime
