#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown --release

rm -rf dist
mkdir dist
cp -r resources dist/assets
cp target/wasm32-unknown-unknown/release/tetris-troll.wasm dist/
cp web/index.html dist/
ls -lh dist