#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <string>

#include "matrix.h"


TEST_CASE("constructing and inspecting a 4x4 matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {1, 2, 3, 4, 5.5, 6.5, 7.5, 8.5, 9, 10, 11, 12, 13.5, 14.5, 15.5, 16.5}};

    REQUIRE(m.at(0, 0) == 1);
    REQUIRE(m.at(0, 3) == 4);
    REQUIRE(m.at(1, 0) == 5.5);
    REQUIRE(m.at(1, 2) == 7.5);
    REQUIRE(m.at(2, 2) == 11);
    REQUIRE(m.at(3, 0) == 13.5);
    REQUIRE(m.at(3, 2) == 15.5);
}
