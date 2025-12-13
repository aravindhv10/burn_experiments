#!/bin/sh
cd "$('dirname' -- "${0}")"
IMAGE_NAME='7_rust_libtorch_demo'
mkdir -pv -- './build'
H="$(cat './Dockerfile' | sha512sum | cut -d ' ' -f1)"
test -e "./build/${H}" && exit '0'
CMD='sudo -A docker'
which buildah && CMD='buildah'
${CMD} build -t "${IMAGE_NAME}" -f './Dockerfile' . && touch "./build/${H}"
exit '0'
