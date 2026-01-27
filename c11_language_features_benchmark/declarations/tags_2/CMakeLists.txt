# CMakeLists.txt for tags_2
cmake_minimum_required(VERSION 3.10)

project(tags_2 LANGUAGES C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_executable(tags_2 tagtags_2s_c.c)
