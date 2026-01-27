# CMakeLists.txt for struct_and_union_2
cmake_minimum_required(VERSION 3.10)

project(struct_and_union_2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(struct_and_union_2 struct_and_union_2.c)
