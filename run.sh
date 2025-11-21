#!/bin/sh
cd "$(dirname -- "${0}")"
make -j
export RUSTFLAGS="-C target-cpu=native"
cargo run --release
