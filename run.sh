#!/bin/sh
cd "$(dirname -- "${0}")"
make -j install
export RUSTFLAGS="-C target-cpu=native"
cargo run --bin infer-server --release
