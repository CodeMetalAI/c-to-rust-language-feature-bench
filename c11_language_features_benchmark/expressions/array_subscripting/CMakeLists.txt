# CMakeLists.txt for array_subscripting
cmake_minimum_required(VERSION 3.10)

project(array_subscripting LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(array_subscripting array_subscripting.c)
