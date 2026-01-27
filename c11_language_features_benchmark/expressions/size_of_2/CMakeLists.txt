# CMakeLists.txt for size_of_2
cmake_minimum_required(VERSION 3.10)

project(size_of_2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(size_of_2 size_of_2.c)
