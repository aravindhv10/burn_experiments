cmake_minimum_required(VERSION 3.18 FATAL_ERROR)

project(aoti_example VERSION 1.0 DESCRIPTION "Interface for inferring with libtorch exposed to rust")

find_package(Torch REQUIRED)

add_executable(aoti_example src/test.cpp)

add_library(mytorch SHARED src/main.cpp)

set_target_properties(mytorch PROPERTIES VERSION ${PROJECT_VERSION})

target_link_libraries(aoti_example "${TORCH_LIBRARIES}")

set_property(TARGET aoti_example PROPERTY CXX_STANDARD 17)
set_property(TARGET mytorch PROPERTY CXX_STANDARD 17)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3 -mtune=native -march=native -flto")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -O3 -mtune=native -march=native -flto")
