#!/bin/sh
cd "$(dirname -- "${0}")"
export LIBTORCH="$(realpath .)/libtorch"
cargo run
