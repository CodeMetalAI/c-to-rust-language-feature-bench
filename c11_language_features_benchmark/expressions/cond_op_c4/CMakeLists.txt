# CMakeLists.txt for cond_op_c4
cmake_minimum_required(VERSION 3.10)

project(cond_op_c4 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(cond_op_c4 cond_op_c4.c)
