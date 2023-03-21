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

add_executable(ray_test ray_test.cpp)
target_link_libraries(ray_test PUBLIC ${CONAN_LIBS})
target_include_directories(ray_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME ray_test COMMAND ray_test)

add_executable(sphere_test sphere_test.cpp)
target_link_libraries(sphere_test PUBLIC ${CONAN_LIBS})
target_include_directories(sphere_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME sphere_test COMMAND sphere_test)

add_executable(intersection_test intersection_test.cpp)
target_link_libraries(intersection_test PUBLIC ${CONAN_LIBS})
target_include_directories(intersection_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME intersection_test COMMAND intersection_test)

add_executable(light_test light_test.cpp)
target_link_libraries(light_test PUBLIC ${CONAN_LIBS})
target_include_directories(light_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME light_test COMMAND light_test)

add_executable(material_test material_test.cpp)
target_link_libraries(material_test PUBLIC ${CONAN_LIBS})
target_include_directories(material_test PRIVATE $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}>/include $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}>)
add_test(NAME material_test COMMAND material_test)
