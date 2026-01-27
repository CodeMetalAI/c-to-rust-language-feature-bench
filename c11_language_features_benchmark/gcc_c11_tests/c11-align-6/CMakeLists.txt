# CMakeLists.txt for c11-align-6
cmake_minimum_required(VERSION 3.10)

project(c11-align-6 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(c11-align-6 c11-align-6.c)
