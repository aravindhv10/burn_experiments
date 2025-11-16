#!/bin/sh
RUN_CONTAINER () {
    CMD='sudo -A docker'
    which podman && CMD='podman'
    ${CMD} run -it --rm -v "$(realpath .):/data" debtestrustzshhelixpytorch zsh
}
