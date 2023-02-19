cmake_minimum_required(VERSION 3.24)
project(raytracer_challenge)

set(CMAKE_CXX_STANDARD 23)

include(conanbuildinfo.cmake)
conan_basic_setup()

add_executable(raytracer_challenge main.cpp)

enable_testing()
message("Building Tests. Be sure to check out test/constexpr_tests for constexpr testing")
add_subdirectory(tests)