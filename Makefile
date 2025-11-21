CC=g++

.PHONY install: build/libmytorch.so
	cp -vf -- build/libmytorch.so /lib/

src/all.hpp: src/main.hpp
	$(CC) src/main.hpp -o src/all.hpp -E -I/usr/include/torch/csrc/api/include/

build/main.o: src/main.cpp src/all.hpp
	mkdir -pv -- ./build ./lib
	$(CC) src/main.cpp -fPIC -c -o build/main.o -O3 -march=x86-64-v3 -mtune=native

build/libmytorch.so: build/main.o
	$(CC) build/main.o -o build/libmytorch.so -fPIC -shared -ltorch -ltorch_cpu
