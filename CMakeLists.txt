cmake_minimum_required(VERSION 3.24)
project(raytracer_challenge)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic -Wunused -Wconversion")

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_library(${PROJECT_NAME} INTERFACE)
target_include_directories(${PROJECT_NAME} INTERFACE include/)

add_executable(main main.cpp)
target_include_directories(main PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
target_link_libraries(main PRIVATE fmt)

enable_testing()
add_subdirectory(tests)
