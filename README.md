# Diagware

## Questions

- **Serial**: how many bytes? including characters?
- **Docs**: installation instructions?


## TODO

- Testing Code on RPi4
- X11 config
- Writing serial to EEPROM


## NOTES

- Always (cargo) run from the project root
- Ensure that `.cargo/config.toml` holds the correct url to the database
  - (needed for compile-time query checking by sqlx)
- **RPi**: username: `pi`, password: `piuser4`



## Procedures


### Login into RPi with SSH

- `ssh <user>@<ip>`


### Setting up Public Key authorization *(optional)*

On the host machine:

- `ssh-keygen -t rsa`
- `ssh-copy-id -i .ssh/id_rsa.pub <user>@<ip>`



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
