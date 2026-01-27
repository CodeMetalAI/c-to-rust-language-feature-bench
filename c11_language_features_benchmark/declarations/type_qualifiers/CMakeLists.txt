# CMakeLists.txt for structure_and_union_spec_b
cmake_minimum_required(VERSION 3.10)

project(type_qualifiers LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(type_qualifiers type_qualifiers.c)
