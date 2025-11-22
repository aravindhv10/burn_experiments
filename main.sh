#!/bin/sh
H="$(sha512sum ./main.py | cut -d ' ' -f1)"
test -e "${H}.pt2" && exit '0'
./main.py "${H}.pt2"
ln -vfs -- "${H}.pt2" "./model_input.pt2"
exit '0'
