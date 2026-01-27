# CMakeLists.txt for fun_defs_2
cmake_minimum_required(VERSION 3.10)

project(fun_defs_2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(fun_defs_2 fun_defs_2.c)
