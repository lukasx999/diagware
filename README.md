# Mixed Signal Building Blocks - Diagware

## TODO
- fix login textedit

## NOTES

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
