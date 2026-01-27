# CMakeLists.txt for restrict_def_4b
cmake_minimum_required(VERSION 3.10)

project(restrict_def_4b LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(restrict_def_4b restrict_def_4b.c)
