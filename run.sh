#!/bin/sh
cd "$(dirname -- "${0}")"

export ROCR_VISIBLE_DEVICES=0
./infer-server
