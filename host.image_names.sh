#!/bin/sh
IMAGE_NAME='burnexps'

RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm \
        -v "$(realpath .):/data" \
        "${IMAGE_NAME}" zsh ;
}
