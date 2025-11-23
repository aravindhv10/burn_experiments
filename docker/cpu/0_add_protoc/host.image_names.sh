#!/bin/sh
# IMAGE_NAME='debtestrustzshhelixpytorch2'
IMAGE_NAME='libtorchapi'

BUILD_CONTAINER () {
    CMD='sudo -A docker'
    which buildah && CMD='buildah'
    ${CMD} build -t "${IMAGE_NAME}" -f "./Dockerfile" .
}

RUN_CONTAINER () {
    mkdir -pv -- './cache'
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm \
        -v "$(realpath .):/data" \
        -v "$(realpath .)/cache:/root/.cache" \
        "${IMAGE_NAME}" zsh ;
}
