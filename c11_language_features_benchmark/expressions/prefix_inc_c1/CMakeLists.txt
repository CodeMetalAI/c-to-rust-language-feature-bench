# CMakeLists.txt for prefix_inc_c1
cmake_minimum_required(VERSION 3.10)

project(prefix_inc_c1 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(prefix_inc_c1 prefix_inc_c1.c)
