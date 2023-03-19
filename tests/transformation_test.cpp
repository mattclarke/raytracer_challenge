#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <numbers>
#include <string>

#include "tuple.h"
#include "transformations.h"


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

TEST_CASE("reflection is scaling by a negative value", "[transformation]" ) {
    auto t = scaling(-1, 1, 1);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(-2, 3, 4));
}

TEST_CASE("rotating a point around the x axis", "[transformation]" ) {
    auto p = point(0, 1, 0);
    auto half_quarter = rotation_x(std::numbers::pi_v<float> / 4);
    auto full_quarter = rotation_x(std::numbers::pi_v<float> / 2);

    REQUIRE(half_quarter * p == point(0, std::sqrt(2) / 2, std::sqrt(2) / 2));
    REQUIRE(full_quarter * p == point(0, 0, 1));
}

TEST_CASE("the inverse of an x-rotation rotates in the opposite direction", "[transformation]" ) {
    auto p = point(0, 1, 0);
    auto half_quarter = rotation_x(std::numbers::pi_v<float> / 4);
    auto inv = inverse(half_quarter);

    REQUIRE(inv * p == point(0, std::sqrt(2) / 2, -std::sqrt(2) / 2));
}

TEST_CASE("rotating a point about the y axis", "[transformation]" ) {
    auto p = point(0, 0, 1);
    auto half_quarter = rotation_y(std::numbers::pi_v<float> / 4);
    auto full_quarter = rotation_y(std::numbers::pi_v<float> / 2);

    REQUIRE(half_quarter * p == point(std::sqrt(2) / 2, 0, std::sqrt(2) / 2));
    REQUIRE(full_quarter * p == point(1, 0, 0));
}

TEST_CASE("rotating a point about the z axis", "[transformation]" ) {
    auto p = point(0, 1, 0);
    auto half_quarter = rotation_z(std::numbers::pi_v<float> / 4);
    auto full_quarter = rotation_z(std::numbers::pi_v<float> / 2);

    REQUIRE(half_quarter * p == point(-std::sqrt(2) / 2, std::sqrt(2) / 2, 0));
    REQUIRE(full_quarter * p == point(-1, 0, 0));
}

TEST_CASE("a shearing transformation moves x in proportion to y", "[transformation]" ) {
    auto t = shearing(1, 0, 0, 0, 0, 0);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(5, 3, 4));
}

TEST_CASE("a shearing transformation moves x in proportion to z", "[transformation]" ) {
    auto t = shearing(0, 1, 0, 0, 0, 0);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(6, 3, 4));
}

TEST_CASE("a shearing transformation moves y in proportion to x", "[transformation]" ) {
    auto t = shearing(0, 0, 1, 0, 0, 0);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(2, 5, 4));
}

TEST_CASE("a shearing transformation moves y in proportion to z", "[transformation]" ) {
    auto t = shearing(0, 0, 0, 1, 0, 0);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(2, 7, 4));
}

TEST_CASE("a shearing transformation moves z in proportion to x", "[transformation]" ) {
    auto t = shearing(0, 0, 0, 0, 1, 0);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(2, 3, 6));
}

TEST_CASE("a shearing transformation moves z in proportion to y", "[transformation]" ) {
    auto t = shearing(0, 0, 0, 0, 0, 1);
    auto p = point(2, 3, 4);

    REQUIRE(t * p == point(2, 3, 7));
}

TEST_CASE("individual transformations are applied in sequence", "[transformation]" ) {
    auto p = point(1, 0, 1);
    auto a = rotation_x(std::numbers::pi_v<float> / 2 );
    auto b = scaling(5, 5, 5);
    auto c = translation(10, 5, 7);

    auto p2 = a * p;
    
    REQUIRE(p2 == point(1, -1, 0));

    auto p3 = b * p2;

    REQUIRE(p3 == point(5, -5, 0));

    auto p4 = c * p3;

    REQUIRE(p4 == point(15, 0, 7));
}

TEST_CASE("chained transformations must be applied in reverse order", "[transformation]" ) {
    auto p = point(1, 0, 1);
    auto a = rotation_x(std::numbers::pi_v<float> / 2 );
    auto b = scaling(5, 5, 5);
    auto c = translation(10, 5, 7);

    auto t = c * b * a;

    REQUIRE(t * p == point(15, 0, 7));
}
