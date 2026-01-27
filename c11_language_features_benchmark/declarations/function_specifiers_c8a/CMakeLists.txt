# CMakeLists.txt for function_specifiers_c8a
cmake_minimum_required(VERSION 3.10)

project(function_specifiers_c8a LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(function_specifiers_c8a function_specifiers_c8a.c)
