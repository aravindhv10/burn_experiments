#!/bin/sh
cd "$(dirname -- "${0}")"
make -j install
export RUSTFLAGS="-C target-cpu=native"
export RUSTFLAGS="ROCR_VISIBLE_DEVICES=1"
cargo run --bin infer-server --release
