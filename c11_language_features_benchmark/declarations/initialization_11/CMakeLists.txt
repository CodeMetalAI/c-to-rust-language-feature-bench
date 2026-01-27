# CMakeLists.txt for initialization_11
cmake_minimum_required(VERSION 3.10)

project(initialization_11 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(initialization_11 initialization_11.c)
