# CMakeLists.txt for scope_of_macro_3
cmake_minimum_required(VERSION 3.10)

project(scope_of_macro_3 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(scope_of_macro_3 scope_of_macro_3.c)
