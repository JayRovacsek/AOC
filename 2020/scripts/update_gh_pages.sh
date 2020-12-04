#!/bin/sh

cd 2020
wasm-pack build
cargo bench
cp -rv ./target/criterion ./www
cd www
npm install
cd ../../
git subtree push --prefix 2020/www origin gh-pages