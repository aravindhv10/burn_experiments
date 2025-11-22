#!/bin/sh
H="$(cat './model_input.pt2' './compile.py' | sha512sum | cut -d ' ' -f1)"
test -e "${H}.pt2" && exit '0'
./compile.py './model_input.pt2' "./${H}.pt2"
ln -vfs -- "./${H}.pt2" './model_output.pt2'
exit '0'
