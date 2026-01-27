# CMakeLists.txt for add_op_1b
cmake_minimum_required(VERSION 3.10)

project(add_op_1b LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(add_op_1b add_op_1b.c)
