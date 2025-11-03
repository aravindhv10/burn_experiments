#!/bin/sh
CMD='sudo -A docker'
which buildah && CMD='buildah'
${CMD} build -t rust_conv - < ./Dockerfile_rust_conv
