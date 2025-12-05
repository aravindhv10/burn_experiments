#!/bin/sh
cd "$(dirname -- "${0}")"
SRC="$(realpath .)"
BLD="${SRC}/../build"
mkdir -pv -- "${BLD}"
cd "${BLD}"

export ROCR_VISIBLE_DEVICES=0
./infer-server
