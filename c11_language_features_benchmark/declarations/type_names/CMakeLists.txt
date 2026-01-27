# CMakeLists.txt for type_names
cmake_minimum_required(VERSION 3.10)

project(type_names LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(type_names type_names.c)
