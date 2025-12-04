#!/bin/sh
cd "$(dirname -- "${0}")"
mkdir -pv -- build
cd build
cmake ..
make -j4
make install
cd ..
H="$(sha512sum ./main.py | cut -d ' ' -f1)"
mkdir -pv -- "build/${H}"
test -e "build/${H}/model.pt2" && exit '0'
./main.py "build/${H}/model.pt2"
ln -vfs -- "build/${H}/model.pt2" "./model.pt2"
exit '0'
