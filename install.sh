#!/usr/bin/env bash
set -euxo pipefail

cargo build --release
cp ./target/release/diagware .
sudo chown root: ./diagware
sudo chmod 4755 ./diagware # Setuid

# bin=/home/lukas/diagware
# cp ./target/debug/diagware-rs $bin
# sudo chown root: $bin
# sudo chmod 4755 $bin
