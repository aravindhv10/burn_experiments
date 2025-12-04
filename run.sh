#!/bin/sh
cd "$(dirname -- "${0}")"
export RUSTFLAGS="-C target-cpu=native"
export ROCR_VISIBLE_DEVICES=1
cargo run --bin infer-server --release
