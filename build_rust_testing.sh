#!/bin/sh
CMD='sudo -A docker'
which buildah && CMD='buildah'
echo "${CMD} build -t rust_testing - < ./Dockerfile_rust_testing"
${CMD} build -t rust_testing - < ./Dockerfile_rust_testing
