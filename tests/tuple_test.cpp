#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include "tuple.h"

TEST_CASE("a tuple with w=1 is a point", "[tuple]" ) {
    Tuple tuple{4.3, -4.2, 3.1, 1.0};

    REQUIRE(tuple.x == 4.3);
    REQUIRE(tuple.y == -4.2);
    REQUIRE(tuple.z == 3.1);
    REQUIRE(tuple.w == 1.0);
    REQUIRE(tuple.is_point() == true);
    REQUIRE(tuple.is_vector() == false);
}

TEST_CASE("a tuple with w=0 is a vector", "[tuple]" ) {
    Tuple tuple{4.3, -4.2, 3.1, 0.0};

    REQUIRE(tuple.x == 4.3);
    REQUIRE(tuple.y == -4.2);
    REQUIRE(tuple.z == 3.1);
    REQUIRE(tuple.w == 0.0);
    REQUIRE(tuple.is_point() == false);
    REQUIRE(tuple.is_vector() == true);
}

TEST_CASE("point() creates tuple with w=1", "[tuple]" ) {
    Tuple tuple = point(4, -4, 3);

    REQUIRE(tuple == Tuple{4, -4, 3, 1.0});
}

TEST_CASE("vector() creates vector with w=0", "[tuple]" ) {
    Tuple tuple = vector(4, -4, 3);

    REQUIRE(tuple == Tuple{4, -4, 3, 0.0});
}

TEST_CASE("adding two tuples", "[tuple]" ) {
    Tuple t1{3, -2, 5, 1};
    Tuple t2{-2, 3, 1, 0};

    REQUIRE(t1 + t2 == Tuple{1, 1, 6, 1.0});
}

TEST_CASE("subtracting two points", "[tuple]" ) {
    auto pt1 = point(3, 2, 1);
    auto pt2 = point(5, 6, 7);

    REQUIRE(pt1 - pt2 == vector(-2, -4, -6));
}

TEST_CASE("subtracting a vector from a point", "[tuple]" ) {
    auto pt = point(3, 2, 1);
    auto vec = vector(5, 6, 7);

    REQUIRE(pt - vec == point(-2, -4, -6));
}

TEST_CASE("subtracting two vectors", "[tuple]" ) {
    auto vec1 = vector(3, 2, 1);
    auto vec2 = vector(5, 6, 7);

    REQUIRE(vec1 - vec2 == vector(-2, -4, -6));
}

TEST_CASE("subtracting a vector from the zero vector", "[tuple]" ) {
    auto zero = vector(0, 0, 0);
    auto vec = vector(1, -2, 3);

    REQUIRE(zero - vec == vector(-1, 2, -3));
}

TEST_CASE("negating a tuple", "[tuple]" ) {
    auto tuple = Tuple{1, -2, 3, -4};

    REQUIRE(-tuple == Tuple{-1, 2, -3, 4});
}

TEST_CASE("multiplying a tuple by a scalar", "[tuple]" ) {
    auto tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple * 3.5 == Tuple{3.5, -7, 10.5, -14});
}

TEST_CASE("multiplying a tuple by a fraction", "[tuple]" ) {
    auto tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple * 0.5 == Tuple{0.5, -1, 1.5, -2});
}

TEST_CASE("dividing a tuple by a scalar", "[tuple]" ) {
    auto tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple / 2 == Tuple{0.5, -1, 1.5, -2});
}

TEST_CASE("computing the magnitude of vector (1, 0, 0)", "[tuple]" ) {
    auto vec = vector(1, 0, 0);

    REQUIRE(magnitude(vec) == 1);
}

TEST_CASE("computing the magnitude of vector (0, 1, 0)", "[tuple]" ) {
    auto vec = vector(0, 1, 0);

    REQUIRE(magnitude(vec) == 1);
}

TEST_CASE("computing the magnitude of vector (0, 0, 1)", "[tuple]" ) {
    auto vec = vector(0, 0, 1);

    REQUIRE(magnitude(vec) == 1);
}

TEST_CASE("computing the magnitude of vector (1, 2, 3)", "[tuple]" ) {
    auto vec = vector(1, 2, 3);

    REQUIRE(magnitude(vec) == std::sqrt(14));
}

TEST_CASE("normalising vector (4, 0, 0) gives (1, 0, 0)", "[tuple]" ) {
    auto vec = vector(4, 0, 0);

    REQUIRE(normalise(vec) == vector(1, 0, 0));
}

TEST_CASE("normalising vector (1, 2, 3)", "[tuple]" ) {
    using Catch::Matchers::WithinRel;
    auto vec = vector(1, 2, 3);

    auto result = normalise(vec);

    REQUIRE_THAT(result.x, WithinRel(0.26726, 0.00001));
    REQUIRE_THAT(result.y, WithinRel(0.53452, 0.00001));
    REQUIRE_THAT(result.z, WithinRel(0.80178, 0.00001));
}

TEST_CASE("the magnitude of a normalised vector", "[tuple]" ) {
    auto vec = vector(1, 2, 3);

    REQUIRE(magnitude(normalise(vec)) == 1);
}

TEST_CASE("the dot product of two tuples", "[tuple]" ) {
    auto vec1 = vector(1, 2, 3);
    auto vec2 = vector(2, 3, 4);

    REQUIRE(dot(vec1, vec2) == 20);
}

TEST_CASE("the cross product of two vectors", "[tuple]" ) {
    auto vec1 = vector(1, 2, 3);
    auto vec2 = vector(2, 3, 4);

    REQUIRE(cross(vec1, vec2) == vector(-1, 2, -1));
    REQUIRE(cross(vec2, vec1) == vector(1, -2, 1));
}

TEST_CASE("colours are (red, green, blue) tuples", "[colour]" ) {
    auto c = Colour{-0.5, 0.4, 1.7};

    REQUIRE(c.red == -0.5);
    REQUIRE(c.green == 0.4);
    REQUIRE(c.blue == 1.7);
}

TEST_CASE("adding colours", "[colour]" ) {
    auto c1 = Colour{0.9, 0.6, 0.75};
    auto c2 = Colour{0.7, 0.1, 0.25};

    REQUIRE(c1 + c2 == Colour{1.6, 0.7, 1.0});
}

TEST_CASE("subtracting colours", "[colour]" ) {
    auto c1 = Colour{0.9, 0.6, 0.75};
    auto c2 = Colour{0.7, 0.1, 0.25};

    REQUIRE(c1 - c2 == Colour{0.2, 0.5, 0.5});
}

TEST_CASE("multiplying a colour by a scalar", "[colour]" ) {
    auto c = Colour{0.2, 0.3, 0.4};

    REQUIRE(c * 2 == Colour{0.4, 0.6, 0.8});
}

TEST_CASE("multiplying colours", "[colour]" ) {
    auto c1 = Colour{1, 0.2, 0.4};
    auto c2 = Colour{0.9, 1, 0.1};

    REQUIRE(c1 * c2 == Colour{0.9, 0.2, 0.04});
}
