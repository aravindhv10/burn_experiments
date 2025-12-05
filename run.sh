#!/bin/sh
cd "$(dirname -- "${0}")"
export RUSTFLAGS="-C target-cpu=native"
export ROCR_VISIBLE_DEVICES=0
bindgen './src/export.hpp' > './src/export.rs'
cargo run --bin infer-server --release
