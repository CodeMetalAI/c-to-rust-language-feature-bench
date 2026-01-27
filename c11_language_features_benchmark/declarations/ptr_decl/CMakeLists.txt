# CMakeLists.txt for ptr_decl
cmake_minimum_required(VERSION 3.10)

project(ptr_decl LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(ptr_decl ptr_decl.c)
