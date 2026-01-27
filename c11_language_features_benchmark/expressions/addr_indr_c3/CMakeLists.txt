# CMakeLists.txt for addr_indr_c3
cmake_minimum_required(VERSION 3.10)

project(addr_indr_c3 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(addr_indr_c3 addr_indr_c3.c)
