#!/bin/sh
CMD='sudo -A docker'
which buildah && CMD='buildah'
${CMD} build -t rust_libtorch - < ./Dockerfile_rust_libtorch
