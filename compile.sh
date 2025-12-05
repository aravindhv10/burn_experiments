#!/bin/sh
cd "$(dirname -- "${0}")"
SRC="$(realpath .)"
BLD="${SRC}/../build"
mkdir -pv -- "${BLD}"

cd "${BLD}"
cmake "${SRC}"
make -j4
make install

cd "${SRC}"
export RUSTFLAGS="-C target-cpu=native"
bindgen './src/export.hpp' > './src/export.rs'
cargo build --bin infer-server --release
cp -vf -- 'target/release/infer-server' "${BLD}/"

cd "${SRC}"
H="$(sha512sum ./main.py | cut -d ' ' -f1)"
mkdir -pv -- "${BLD}/${H}"
test -e "${BLD}/${H}/model.pt2" || ./main.py "${BLD}/${H}/model.pt2"
ln -vfs -- "${BLD}/${H}/model.pt2" "${BLD}/model.pt2"

exit '0'
