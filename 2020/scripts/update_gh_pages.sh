#!/bin/sh

cd 2020 
wasm-pack build
cd www
npm install
cd ../../
git subtree push --prefix 2020/www origin gh-pages