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

TEST_CASE("muliplying a matrix by the identity matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {0, 1, 2, 4, 1, 2, 4, 8, 2, 4, 8, 16, 4, 8, 16, 32}};
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};

    REQUIRE(m * idm == m);
}

TEST_CASE("multiplying the identity matrix by a tuple", "[matrix]" ) {
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};
    auto t = Tuple{1, 2, 3, 4};

    REQUIRE(idm * t == t);
}

TEST_CASE("transposing a matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {0, 9, 3, 0,
                           9, 8, 0, 8,
                           1, 8, 5, 3,
                           0, 0, 5, 8}};

    auto expected = Matrix{4, 4, {0, 9, 1, 0,
                                  9, 8, 8, 0,
                                  3, 0, 5, 5,
                                  0, 8, 3, 8}};
    REQUIRE(transpose(m) == expected);
}

TEST_CASE("transposing the identity matrix", "[matrix]" ) {
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};

    REQUIRE(transpose(idm) == idm);
}

TEST_CASE("calculating the determinant of a 2x2 matrix", "[matrix]" ) {
    auto m = Matrix{2, 2, {1, 5, -3, 2}};

    REQUIRE(determinant(m) == 17);
}

TEST_CASE("a submatrix of a 3x3 matrix is a 2x2 matrix", "[matrix]" ) {
    auto m = Matrix{3, 3, {1, 5, 0, 3, 2, 7, 0, 6, -3}};

    auto expected = Matrix{2, 2, {3, 2, 0, 6}};

    REQUIRE(submatrix(m, 0, 2) == expected);
}

TEST_CASE("a submatrix of a 4x4 matrix is a 3x3 matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {-6, 1, 1, 6, -8, 5, 8, 6, -1, 0, 8, 2, -7, 1, -1, 1}};

    auto expected = Matrix{3, 3, {-6, 1, 6, -8, 8, 6, -7, -1, 1}};

    REQUIRE(submatrix(m, 2, 1) == expected);
}
