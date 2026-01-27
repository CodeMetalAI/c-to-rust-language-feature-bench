# CMakeLists.txt for function_specifiers_c8b
cmake_minimum_required(VERSION 3.10)

project(function_specifiers_c8b LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(function_specifiers_c8b function_specifiers_c8b.c)
