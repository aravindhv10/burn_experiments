cmake_minimum_required(VERSION 3.18 FATAL_ERROR)
project(aoti_example)

find_package(Torch REQUIRED)

add_executable(aoti_example main.cpp out.pt2)

add_custom_command(
    OUTPUT out.pt2
    COMMAND python ${CMAKE_CURRENT_SOURCE_DIR}/main.py
    DEPENDS main.py
)

target_link_libraries(aoti_example "${TORCH_LIBRARIES}")
set_property(TARGET aoti_example PROPERTY CXX_STANDARD 17)
