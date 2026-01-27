# CMakeLists.txt for rel_op_c6a
cmake_minimum_required(VERSION 3.10)

project(rel_op_c6a LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(rel_op_c6a rel_op_c6a.c)
