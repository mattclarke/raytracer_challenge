#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <string>

#include "tuple.h"
#include "matrix.h"


TEST_CASE("multiplying by a translation matrix", "[transformation]" ) {
    auto t = translation(5, -3, 2);
    auto p = point(-3, 4, 5);

    REQUIRE(t * p == point(2, 1, 7));
}

TEST_CASE("multiplying by the inverse of a translation matrix", "[transformation]" ) {
    auto t = translation(5, -3, 2);
    auto inv = inverse(t);
    auto p = point(-3, 4, 5);

    REQUIRE(inv * p == point(-8, 7, 3));
}

TEST_CASE("translation does not affect vectors", "[transformation]" ) {
    auto t = translation(5, -3, 2);
    auto v = vector(-3, 4, 5);

    REQUIRE(t * v == v);
}

TEST_CASE("a scaling matrix applied to a point", "[transformation]" ) {
    auto t = scaling(2, 3, 4);
    auto p = point(-4, 6, 8);

    REQUIRE(t * p == point(-8, 18, 32));
}

TEST_CASE("a scaling matrix applied to a vector", "[transformation]" ) {
    auto t = scaling(2, 3, 4);
    auto v = vector(-4, 6, 8);

    REQUIRE(t * v == vector(-8, 18, 32));
}

TEST_CASE("multiplying by the inverse of a scaling matrix", "[transformation]" ) {
    auto t = scaling(2, 3, 4);
    auto inv = inverse(t);
    auto v = vector(-4, 6, 8);

    REQUIRE(inv * v == vector(-2, 2, 2));
}
