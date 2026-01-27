# CMakeLists.txt for structure_and_union_spec_b
cmake_minimum_required(VERSION 3.10)

project(atomic_type_spec LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(atomic_type_spec atomic_type_spec.c)
