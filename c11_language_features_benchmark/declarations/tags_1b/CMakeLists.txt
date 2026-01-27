# CMakeLists.txt for tags_1b
cmake_minimum_required(VERSION 3.10)

project(tags_1b LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(tags_1b tags_1b.c)
