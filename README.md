# Diagware for Mixed Signal Building Blocks

## Notes

- loop: cycle delay
- Export Logfile to USB
- Document mounting error popup
- Remove libc::setuid() calls
  - run as root?
- isolated HW interfaces
- improved state machine ui

## TODO
- [ ] add loop delay and visual indicator
- [ ] sqlitestudio
- [ ] Visio Drawing & Explanation for Drawing States


## Docs
- ImGui vs Retained-mode GUI
- State Machine diagram + explaination
- State Machine GUI rendering
- State and Ownership diagram (channels, arcs, mutexes, ...)
- R/W EEPROM
- DB Management (table & column layout)



## NOTES

- Bei SQLx Datenbank Fehlern bei cargo build -> .cargo/config.toml (path muss stimmen)
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
- setting effective user id (euid) in main to 1000 (user) to not mess things up accidentally
  - new spawned processes (mount / umount) will inherit uid=0
  - rust std functions will require manually setting euid to 0



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


### Building for Prod
- `cargo b`
- `sudo ./install.sh`
- `~/diagware`


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
