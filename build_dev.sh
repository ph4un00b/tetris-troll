#!/bin/sh

set -ex

cargo build --target wasm32-unknown-unknown --release

rm -rf web/assets
rm -rf web/tetris-troll.wasm

cp -r resources web/assets
cp target/wasm32-unknown-unknown/release/tetris-troll.wasm web/

ls -lh web

basic-http-server web/