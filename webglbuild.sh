#/bin/bash

cargo build --target wasm32-unknown-unknown --release --no-default-features --features web
wasm-bindgen ./target/wasm32-unknown-unknown/release/bload_oath.wasm --out-dir ./web/wasm --no-modules --no-typescript