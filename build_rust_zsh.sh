#!/bin/sh
CMD='sudo -A docker'
which buildah && CMD='buildah'
echo "${CMD} build -t rust_zsh - < ./Dockerfile_rust_zsh"
${CMD} build -t rust_zsh - < ./Dockerfile_rust_zsh
