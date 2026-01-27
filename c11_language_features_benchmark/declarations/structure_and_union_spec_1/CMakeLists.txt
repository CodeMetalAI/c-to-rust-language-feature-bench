# CMakeLists.txt for structure_and_union_spec
cmake_minimum_required(VERSION 3.10)

project(structure_and_union_spec LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(structure_and_union_spec_1 structure_and_union_spec_1.c)
