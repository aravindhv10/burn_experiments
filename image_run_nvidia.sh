#!/bin/sh
cd "$(dirname -- "${0}")"

mkdir -pv -- "${HOME}/BUILD"

IMAGE_NAME='7_rust_libtorch_demo'

RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run \
        --tty \
        --interactive \
        --rm \
        --gpus 'all,"capabilities=compute,utility,video"' \
        --ipc host \
        --ulimit memlock=-1 \
        --ulimit stack=67108864 \
        --shm-size 107374182400 \
        --mount 'type=tmpfs,destination=/data/TMPFS,tmpfs-size=137438953472' \
        -v "$(realpath .):/data/source" \
        -v "${HOME}/BUILD:/data/build" \
        -v "CACHE:/usr/local/cargo/registry" \
        -v "CACHE:/root/.cache" \
        -v "CACHE:/root/.triton" \
        "${IMAGE_NAME}" zsh \
    ;
}

RUN_CONTAINER
