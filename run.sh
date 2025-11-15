#!/bin/sh
cd "$(dirname -- "${0}")"
# export CXXFLAGS='-I/usr/include/torch/csrc/api/include/'
# export CXXFLAGS='-laoti_custom_ops -lbackend_with_compiler -lc10 -lgomp-98b21ff3 -ljitbackend_test -lnnapi_backend -lshm -ltorch -ltorch_cpu -ltorch_global_deps -ltorch_python -ltorchbind_test'
cargo run
