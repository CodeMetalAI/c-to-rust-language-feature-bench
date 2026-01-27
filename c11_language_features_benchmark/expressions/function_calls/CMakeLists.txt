# CMakeLists.txt for function_calls
cmake_minimum_required(VERSION 3.10)

project(function_calls LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(function_calls function_calls.c)
