# CMakeLists.txt for storage_duration_c6
cmake_minimum_required(VERSION 3.10)

project(storage_duration_c6 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(storage_duration_c6 storage_duration_c6.c)
