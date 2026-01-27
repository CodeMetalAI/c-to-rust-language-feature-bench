# CMakeLists.txt for eq_op_c3
cmake_minimum_required(VERSION 3.10)

project(eq_op_c3 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(eq_op_c3 eq_op_c3.c)
