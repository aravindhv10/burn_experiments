#!/bin/sh
cd "$('dirname' -- "${0}")"
IMAGE_NAME='7_rust_libtorch_demo'
mkdir -pv -- './build'

H="$(cat './CMakeLists.txt' './Cargo.toml' './Dockerfile' './build.rs' './compile.py' './compile.sh' './curl.sh' './infer.proto' './infer.sh' './main.py' './run.sh' './src/client.rs' './src/export.hpp' './src/main.cpp' './src/main.hpp' './src/main.rs' './src/model.rs' './src/mylib.rs' './src/test.cpp' | sha512sum | cut -d ' ' -f1)"

test -e "./build/${H}" && exit '0'
CMD='sudo -A docker'
which buildah && CMD='buildah'
${CMD} build -t "${IMAGE_NAME}" -f './Dockerfile' . && touch "./build/${H}"
exit '0'
