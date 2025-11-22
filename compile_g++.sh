#!/bin/sh
mkdir -pv -- './lib/' './tmp/'

bear -- \
    clang++ \
    './src/main.cpp' -o './tmp/main.o' \
    -c -fPIC \
    '-I/usr/include/torch/csrc/api/include/' \
;
