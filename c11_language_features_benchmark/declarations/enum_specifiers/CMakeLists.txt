# CMakeLists.txt for structure_and_union_spec_2
cmake_minimum_required(VERSION 3.10)

project(enum_specifiers LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(enum_specifiers enum_specifiers.c)
