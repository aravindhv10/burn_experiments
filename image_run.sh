#!/bin/sh
cd "$(dirname -- "${0}")"

IMAGE_NAME='libtorchapi'

RUN_CONTAINER () {
    mkdir -pv -- './cache'
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm \
        -v "$(realpath .):/data" \
        -v "$(realpath .)/cache:/root/.cache" \
        "${IMAGE_NAME}" zsh ;
}

RUN_CONTAINER
