# Diagware

## Questions

- **Serial**: how many bytes? including characters?


## Notes

- Always (cargo) run from the project root

## Procedures

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




## Dependencies

- **sqlx**
  - Database
- **egui**
  - UI
- **rppal**
  - Raspberry Pi4 IO (GPIO, I2C, SPI, ...)
- **tokio**
  - Rust Async runtime
