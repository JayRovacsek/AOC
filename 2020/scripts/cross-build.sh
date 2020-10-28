#!/bin/sh

cross build --release --target x86_64-pc-windows-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target aarch64-linux-android
cross build --release --target powerpc-unknown-linux-gnu
cargo build --release --target wasm32-wasi 

tar -cvzf ./release/x86_64-windows.tar.gz ./target/x86_64-pc-windows-gnu/release/aoc-2020.exe
tar -czvf ./release/aarch64-linux.tar.gz ./target/aarch64-unknown-linux-gnu/release/aoc-2020
tar -czvf ./release/aarch64-android.tar.gz ./target/aarch64-linux-android/release/aoc-2020
tar -czvf ./release/powerpc-linux.tar.gz ./target/powerpc-unknown-linux-gnu/release/aoc-2020
tar -czvf ./release/wasm.tar.gz ./target/wasm32-wasi/release/aoc-2020.wasm