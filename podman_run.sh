#!/bin/sh
podman run -it --rm -v "$(realpath .):/data" rust_tch_rs zsh
