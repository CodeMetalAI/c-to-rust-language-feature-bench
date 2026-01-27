# CMakeLists.txt for mul_op_c6
cmake_minimum_required(VERSION 3.10)

project(mul_op_c6 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(mul_op_c6 mul_op_c6.c)
