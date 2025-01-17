#!/usr/bin/env bash
set -euxo pipefail

bin=~/diagware

cargo build --release
cp ./target/release/diagware-rs $bin
sudo chown root: $bin
sudo chmod 4755 $bin
