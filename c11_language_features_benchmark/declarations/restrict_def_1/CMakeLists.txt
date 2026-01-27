# CMakeLists.txt for restrict_def_1
cmake_minimum_required(VERSION 3.10)

project(restrict_def_1 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(restrict_def_1 restrict_def_1.c)
