#!/bin/bash
# Cross-compile DarkWrite pour Windows & Linux depuis Linux
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
strip target/x86_64-pc-windows-gnu/release/darkwrite.exe
echo "Binaire Windows généré : target/x86_64-pc-windows-gnu/release/darkwrite.exe"
cargo build --release
strip target/release/darkwrite
echo "Binaire Linux généré : target/release/darkwrite"