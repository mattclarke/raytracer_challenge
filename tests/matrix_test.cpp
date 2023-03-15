#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <string>

#include "matrix.h"
#include "tuple.h"


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

TEST_CASE("a 2x2 matrix ought to be representable", "[matrix]" ) {
    auto m = Matrix{2, 2, {-3, 5, 1, -2}};

    REQUIRE(m.at(0, 0) == -3);
    REQUIRE(m.at(0, 1) == 5);
    REQUIRE(m.at(1, 0) == 1);
    REQUIRE(m.at(1, 1) == -2);
}

TEST_CASE("a 3x3 matrix ought to be representable", "[matrix]" ) {
    auto m = Matrix{3, 3, {-3, 5, 0, 1, -2, -7, 0, 1, 1}};

    REQUIRE(m.at(0, 0) == -3);
    REQUIRE(m.at(1, 1) == -2);
    REQUIRE(m.at(2, 2) == 1);
}

TEST_CASE("matrix equality with identical matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};

    REQUIRE(m1 == m2);
}

TEST_CASE("matrix equality with different matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1}};

    REQUIRE(m1 != m2);
}

TEST_CASE("multiplying two matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {-2, 1, 2, 3, 3, 2, 1, -1, 4, 3, 6, 5, 1, 2, 7, 8}};

    auto expected = Matrix{4, 4, {20, 22, 50, 48, 44, 54, 114, 108, 40, 58, 110, 102, 16, 26, 46, 42}};

    REQUIRE(m1 * m2 == expected);
}

TEST_CASE("matrix multiplied by a tuple", "[matrix]" ) {
    auto m = Matrix{4, 4, {1, 2, 3, 4, 2, 4, 4, 2, 8, 6, 4, 1, 0, 0, 0, 1}};
    auto t = Tuple{1, 2, 3, 1};

    REQUIRE(m * t == Tuple{18, 24, 33, 1});
}
