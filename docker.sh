#!/bin/sh
cd "$(dirname -- "${0}")"
exec podman run \
    --tty \
    --interactive \
    --rm \
    -v "$(realpath .):/data" \
    --device /dev/kfd \
    --device /dev/dri \
    --security-opt seccomp=unconfined \
    'rocm_rust' \
    bash \
;