#!/bin/sh
cd "$('dirname' -- "${0}")"
cp -apf "${HOME}/important_common_utils" ./
IMAGE_NAME='7_rust_libtorch'
mkdir -pv -- './build'
H="$(cat './Dockerfile' | sha512sum | cut -d ' ' -f1)"
test -e "./build/${H}" && exit '0'
CMD='sudo -A docker'
which buildah && CMD='buildah'
${CMD} build -t "${IMAGE_NAME}" -f './Dockerfile' . && touch "./build/${H}"
exit '0'
