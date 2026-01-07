#!/bin/sh
cargo build --release
sudo cp target/release/rpncalc /usr/local/bin/
echo "rpncalc installed to /usr/local/bin/"