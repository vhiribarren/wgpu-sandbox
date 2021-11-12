#!/usr/bin/env bash

set -xe

BUILD_DIR=target/wasm32-unknown-unknown/debug

cargo build --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir $BUILD_DIR $BUILD_DIR/intro-clock-wgpu.wasm
cp web/index.html $BUILD_DIR
python3 -m http.server -d $BUILD_DIR