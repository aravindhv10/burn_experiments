#!/bin/sh
IMAGE_NAME='debtestrustzshhelixpytorch2'

RUN_CONTAINER () {
    mkdir -pv -- './cache'
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm \
        -v "$(realpath .):/data" \
        -v "$(realpath .)/cache:/root/.cache" \
        "${IMAGE_NAME}" zsh ;
}
