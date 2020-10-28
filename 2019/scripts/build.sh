#!/bin/sh

cargo build --release

tar -cvzf ./release/x86_64-linux.tar.gz ./target/release/aoc-2019
