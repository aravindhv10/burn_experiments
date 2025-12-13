FROM 6_pytorch

RUN mkdir -pv -- '/root/source' '/root/build'

COPY './CMakeLists.txt' '/root/source/CMakeLists.txt'
COPY './Cargo.toml' '/root/source/Cargo.toml'
COPY './build.rs' '/root/source/build.rs'
COPY './compile.py' '/root/source/compile.py'
COPY './compile.sh' '/root/source/compile.sh'
COPY './curl.sh' '/root/curl.sh'
COPY './infer.proto' '/root/source/infer.proto'
COPY './infer.sh' '/root/infer.sh'
COPY './main.py' '/root/source/main.py'
COPY './run.sh' '/root/run.sh'
COPY './src/client.rs' '/root/source/src/client.rs'
COPY './src/export.hpp' '/root/source/src/export.hpp'
COPY './src/main.cpp' '/root/source/src/main.cpp'
COPY './src/main.hpp' '/root/source/src/main.hpp'
COPY './src/main.rs' '/root/source/src/main.rs'
COPY './src/model.rs' '/root/source/src/model.rs'
COPY './src/mylib.rs' '/root/source/src/mylib.rs'
COPY './src/test.cpp' '/root/source/src/test.cpp'

RUN \
    echo 'START Compile c++ parts' \
    && cd '/root/build' \
    && cmake '../source' \
    && make -j4 \
    && make install \
    && echo 'DONE Compile c++ parts' ;


ENV RUSTFLAGS="-C target-cpu=native"
ENV CARGO_TARGET_DIR="/root/build/cargo"

RUN \
    echo 'START Compiling rust parts' \
    && cd '/root/source' \
    && bindgen './src/export.hpp' > './src/export.rs' \
    && cargo build --bin infer-server --release \
    && cargo build --bin infer-client --release \
    && install --compare  "${CARGO_TARGET_DIR}/release/infer-server" '/usr/bin/infer-server' \
    && install --compare  "${CARGO_TARGET_DIR}/release/infer-client" '/usr/bin/infer-client' \
    && echo 'DONE Compiling rust parts' ;
