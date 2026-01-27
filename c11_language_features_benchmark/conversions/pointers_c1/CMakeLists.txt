# CMakeLists.txt for pointers_c1
cmake_minimum_required(VERSION 3.10)

project(pointers_c1 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(pointers_c1 pointers_c1.c)
