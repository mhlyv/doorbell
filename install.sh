#!/bin/sh
prefix="/usr/local"
bin="$prefix/bin"
sounds="$prefix/share/doorbell/sounds"
cargo build --release || exit
sudo systemctl stop doorbell
sudo cp target/release/doorbell "$prefix/bin/"
sudo cp doorbell.service /etc/systemd/system/
sudo systemctl enable --now doorbell
sudo mkdir -p "$sounds"
sudo cp sounds/* "$sounds"
