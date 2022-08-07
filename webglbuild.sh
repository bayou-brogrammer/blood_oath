#/bin/bash

cargo build --target wasm32-unknown-unknown --release --no-default-features
wasm-bindgen ./target/wasm32-unknown-unknown/release/bload_oath.wasm --out-dir ./dist/wasm --no-modules --no-typescript