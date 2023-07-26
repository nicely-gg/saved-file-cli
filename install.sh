#!/usr/bin/bash

# build the cargo project
cargo clean
cargo build --release

# copy the binary into the bin directory
cp ./target/release/savedfile "$HOME/.local/bin"
echo Installed to "$HOME/.local/bin"
