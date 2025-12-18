#!/bin/sh
cd "$(dirname -- "${0}")"
SRC="$(realpath .)"
BLD="${SRC}/../build"
mkdir -pv -- "${BLD}"
BLD="$('realpath' "${BLD}")"

cd "${SRC}"
H="$(cat ./main.py | sha512sum | cut -d ' ' -f1)"
mkdir -pv -- "${HOME}/.cache/${H}"
test -e "${HOME}/.cache/${H}/model.pt2" || './compile_2_trt.py' './model.pt2' "${HOME}/.cache/${H}/model.pt2"
# test -e "${HOME}/.cache/${H}/model.pt2" || ./compile.py './model.ckpt' "${HOME}/.cache/${H}/model.pt2"
ln -vfs -- "${HOME}/.cache/${H}/model.pt2" '/model.pt2'

exit '0'
