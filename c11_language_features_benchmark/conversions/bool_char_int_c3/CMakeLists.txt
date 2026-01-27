# CMakeLists.txt for bool_char_int_c3
cmake_minimum_required(VERSION 3.10)

project(bool_char_int_c3 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(bool_char_int_c3 bool_char_int_c3.c)
