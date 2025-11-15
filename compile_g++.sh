#!/bin/sh
mkdir -pv -- './lib/' './tmp/'
g++ \
    './src/main.cpp' -o './tmp/main.o' \
    -c -fPIC \
    '-I/usr/include/torch/csrc/api/include/' \
;

g++ \
    './tmp.cpp' -o './tmp.exe' \
    './tmp/main.o' \
    -laoti_custom_ops \
    -lbackend_with_compiler \
    -lc10 \
    -lgomp-98b21ff3 \
    -ljitbackend_test \
    -lnnapi_backend \
    -lshm \
    -ltorch \
    -ltorch_cpu \
    -ltorch_global_deps \
    -ltorch_python \
    -ltorchbind_test \
;
