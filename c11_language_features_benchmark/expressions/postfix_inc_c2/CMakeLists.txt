# CMakeLists.txt for postfix_inc_c2
cmake_minimum_required(VERSION 3.10)

project(postfix_inc_c2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(postfix_inc_c2 postfix_inc_c2.c)
