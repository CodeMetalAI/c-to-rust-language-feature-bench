# CMakeLists.txt for external_objs_2
cmake_minimum_required(VERSION 3.10)

project(external_objs_2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(external_objs_2 external_objs_2.c)
