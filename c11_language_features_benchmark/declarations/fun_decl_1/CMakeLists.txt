# CMakeLists.txt for fun_decl_1
cmake_minimum_required(VERSION 3.10)

project(fun_decl_1 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(fun_decl_1 fun_decl_1.c)
