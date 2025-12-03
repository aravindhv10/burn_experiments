CC=clang++

install: all
	install -C build/libmytorch.so /lib/libmytorch.so

all: build/libmytorch.so model_output.pt2 src/export.rs
	echo Done building all

src/main.hpp: src/export.hpp
	touch src/main.hpp

src/all.hpp: src/main.hpp
	$(CC) src/main.hpp -o src/all.hpp -E -I/usr/include/torch/csrc/api/include/

src/main.cpp: src/all.hpp
	touch src/main.cpp

build/main.o: src/main.cpp
	mkdir -pv -- ./build
	$(CC) src/main.cpp -fPIC -c -o build/main.o -O3 -march=x86-64-v3 -mtune=native

build/libmytorch.so: build/main.o
	$(CC) build/main.o -o build/libmytorch.so -fPIC -shared -flto -L/lib/intel64  -L/lib/intel64_win  -L/lib/win-x64  -Wl,-rpath,/lib/intel64:/lib/intel64_win:/lib/win-x64:/opt/rocm-6.4.4/lib:/opt/rocm/lib /usr/lib/libtorch.so /usr/lib/libc10.so /usr/lib/libkineto.a -Wl,--no-as-needed,"/usr/lib/libtorch_cpu.so" -Wl,--as-needed -Wl,--no-as-needed,"/usr/lib/libtorch_hip.so" -Wl,--as-needed /usr/lib/libc10_hip.so /usr/lib/libc10.so /opt/rocm-6.4.4/lib/libMIOpen.so.1.0.60404 /opt/rocm/lib/libhiprtc.so.6.4.60404 -ldl /opt/rocm-6.4.4/lib/libhipblas.so.2.4.60404 /opt/rocm-6.4.4/lib/libhipfft.so.0.1.60404 /opt/rocm-6.4.4/lib/libhiprand.so.1.1.60404 /opt/rocm-6.4.4/lib/librocrand.so.1.1.60404 /opt/rocm-6.4.4/lib/libhipsparse.so.1.1.0.60404 /opt/rocm-6.4.4/lib/libhipsolver.so.0.4.60404 /opt/rocm-6.4.4/lib/librocsolver.so.0.4.60404 /opt/rocm-6.4.4/lib/librocblas.so.4.4.60404 /opt/rocm-6.4.4/lib/libhipblaslt.so.0.12.60404 /opt/rocm/lib/libamdhip64.so.6.4.60404 /opt/rocm-6.4.4/lib/libhipsparselt.so.0.2.60404 -Wl,--no-as-needed,"/usr/lib/libtorch.so" -Wl,--as-needed 

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
