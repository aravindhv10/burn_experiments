#!/bin/sh
cd "$(dirname -- "${0}")"

IMAGE_NAME='debtestrustzshhelixpytorch2'

RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run \
        -it --rm \
        '--device' '/dev/kfd' \
        '--device' '/dev/dri' \
        '--security-opt' 'seccomp=unconfined' \
        -v "$(realpath .):/data/source" \
        -v "CACHE:/data/build" \
        "${IMAGE_NAME}" zsh \
    ;
}

RUN_CONTAINER
