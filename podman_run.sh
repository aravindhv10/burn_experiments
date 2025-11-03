#!/bin/sh
podman run -it --rm -v "$(realpath .):/data" rust_libtorch zsh
