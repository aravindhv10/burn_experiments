#!/bin/sh
cd "$(dirname -- "${0}")"
export LIBTORCH_USE_PYTORCH=1
export LIBTORCH_BYPASS_VERSION_CHECK=1
cargo run
