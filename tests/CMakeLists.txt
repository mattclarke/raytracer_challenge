include(CTest)

add_executable(tuple_test tuple_test.cpp)
target_link_libraries(tuple_test PUBLIC ${CONAN_LIBS})
add_test(NAME tuple_test COMMAND tuple_test)
