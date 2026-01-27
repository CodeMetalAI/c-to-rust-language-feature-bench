# CMakeLists.txt for size_of_3
cmake_minimum_required(VERSION 3.10)

project(size_of_3 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(size_of_3 size_of_3.c)
