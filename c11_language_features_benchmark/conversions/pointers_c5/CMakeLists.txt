# CMakeLists.txt for pointers_c5
cmake_minimum_required(VERSION 3.10)

project(pointers_c5 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(pointers_c5 pointers_c5.c)
