include(CTest)

add_executable(tuple_test tuple_test.cpp)
target_link_libraries(tuple_test PUBLIC ${CONAN_LIBS})
target_include_directories(tuple_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME tuple_test COMMAND tuple_test)

add_executable(canvas_test canvas_test.cpp)
target_link_libraries(canvas_test PUBLIC ${CONAN_LIBS})
target_include_directories(canvas_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME canvas_test COMMAND canvas_test)
