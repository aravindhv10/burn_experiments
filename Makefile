CC=clang++

install: build/libmytorch.so model_output.pt2 src/export.rs
	cp -vf -- build/libmytorch.so /lib/

src/all.hpp: src/main.hpp src/export.hpp
	$(CC) src/main.hpp -o src/all.hpp -E -I/usr/include/torch/csrc/api/include/

build/main.o: src/main.cpp src/all.hpp
	mkdir -pv -- ./build
	$(CC) src/main.cpp -fPIC -c -o build/main.o -O3 -march=x86-64-v3 -mtune=native

build/libmytorch.so: build/main.o
	$(CC) build/main.o -o build/libmytorch.so -fPIC -shared -ltorch -ltorch_cpu -flto

clean:
	rm -rf -- build src/all.hpp target

model_output.pt2: model_input.pt2 compile.py
	./compile.sh
	touch ./model_output.pt2 

src/export.rs: src/export.hpp
	bindgen src/export.hpp > src/export.rs

model_input.pt2: main.py
	./main.sh
	touch ./model_input.pt2
