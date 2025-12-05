#!/bin/sh
cd "$(dirname -- "${0}")"

mkdir -pv -- "${HOME}/BUILD"

IMAGE_NAME='6_pytorch'

RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run \
        -it --rm \
        '--device' '/dev/kfd' \
        '--device' '/dev/dri' \
        '--security-opt' 'seccomp=unconfined' \
        --mount 'type=tmpfs,destination=/data/TMPFS,tmpfs-size=137438953472' \
        -v "$(realpath .):/data/source" \
        -v "${HOME}/BUILD:/data/build" \
        -v "CACHE:/usr/local/cargo/registry" \
        -v "CACHE:/root/.cache" \
        "${IMAGE_NAME}" zsh \
    ;
}

RUN_CONTAINER
