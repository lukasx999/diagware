#!/usr/bin/env bash
set -euxo pipefail

cargo build --release
sudo cp ./target/release/diagware .
sudo chown root: ./diagware
sudo chmod 4755 ./diagware # Setuid bit
