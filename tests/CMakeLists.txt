include(CTest)

add_executable(tuple_test tuple_test.cpp)
target_link_libraries(tuple_test PUBLIC ${CONAN_LIBS})
target_include_directories(tuple_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME tuple_test COMMAND tuple_test)

add_executable(canvas_test canvas_test.cpp)
target_link_libraries(canvas_test PUBLIC ${CONAN_LIBS})
target_include_directories(canvas_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME canvas_test COMMAND canvas_test)

add_executable(matrix_test matrix_test.cpp)
target_link_libraries(matrix_test PUBLIC ${CONAN_LIBS})
target_include_directories(matrix_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME matrix_test COMMAND matrix_test)

add_executable(transformation_test transformation_test.cpp)
target_link_libraries(transformation_test PUBLIC ${CONAN_LIBS})
target_include_directories(transformation_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME transformation_test COMMAND transformation_test)

add_executable(rays_test rays_test.cpp)
target_link_libraries(rays_test PUBLIC ${CONAN_LIBS})
target_include_directories(rays_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME rays_test COMMAND rays_test)
