#!/bin/sh
BUILD_CONTAINER () {
    CMD='sudo -A docker'
    which buildah && CMD='buildah'
    ${CMD} build -t "${2}" -f "./${1}"
}

RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm -v "$(realpath .):/data" rust_final zsh
}
