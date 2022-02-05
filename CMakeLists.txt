cmake_minimum_required(VERSION 3.21)
project("Advent-of-Code")

# Set compile options
set(CMAKE_CXX_EXTENSIONS OFF)
if (MSVC)
    add_compile_options( /W4 /WX )
else()
    add_compile_options( -Wall -Werror -Wconversion )
endif()

add_subdirectory("2018")
