cmake_minimum_required(VERSION 3.24)
project(raytracer_challenge)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic -Wunused -Wconversion")

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_executable(raytracer_challenge main.cpp)

enable_testing()
add_subdirectory(tests)