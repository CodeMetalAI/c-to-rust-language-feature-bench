# CMakeLists.txt for comma_op_c2
cmake_minimum_required(VERSION 3.10)

project(comma_op_c2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(comma_op_c2 comma_op_c2.c)
