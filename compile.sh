#!/bin/sh
cd "$(dirname -- "${0}")"
SRC="$(realpath .)"
mkdir -pv -- '../build'
cd '../build'
BLD="$(realpath .)"
cmake "${SRC}"
make -j4
make install
cd "${SRC}"
H="$(sha512sum ./main.py | cut -d ' ' -f1)"
mkdir -pv -- "${BLD}/${H}"
test -e "${BLD}/${H}/model.pt2" && exit '0'
./main.py "${BLD}/${H}/model.pt2"
ln -vfs -- "${BLD}/${H}/model.pt2" "${BLD}/model.pt2"
exit '0'
