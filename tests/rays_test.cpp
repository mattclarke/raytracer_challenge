#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "rays.h"
#include "sphere.h"
#include "tuple.h"

TEST_CASE("creating and querying a ray", "[rays]" ) {
    auto origin = point(1, 2, 3);
    auto direction = vector(4, 5, 6);
    auto r = Ray{origin, direction};

    REQUIRE(r.origin == origin);
    REQUIRE(r.direction == direction);
}

TEST_CASE("computing a point from a distance", "[rays]" ) {
    auto r = Ray{point(2, 3, 4), vector(1, 0, 0)};

    REQUIRE(position(r, 0) == point(2, 3, 4));
    REQUIRE(position(r, 1) == point(3, 3, 4));
    REQUIRE(position(r, -1) == point(1, 3, 4));
    REQUIRE(position(r, 2.5) == point(4.5, 3, 4));
}

TEST_CASE("spheres are unique", "[rays]" ) {
    auto s1 = Sphere{};
    auto s2 = Sphere{};


    REQUIRE(s1 != s2);
    REQUIRE(s1 == s1);
    REQUIRE(s2 == s2);
}

TEST_CASE("a ray intersects a sphere at two points", "[rays]" ) {
    auto r = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0] == 4.0f);
    REQUIRE(xs[1] == 6.0f);
}

TEST_CASE("a ray intersects a sphere at a tangent", "[rays]" ) {
    auto r = Ray{point(0, 1, -5), vector(0, 0, 1)};
    auto s = Sphere{};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0] == 5.0f);
    REQUIRE(xs[1] == 5.0f);
}

TEST_CASE("a ray misses a sphere", "[rays]" ) {
    auto r = Ray{point(0, 2, -5), vector(0, 0, 1)};
    auto s = Sphere{};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 0);
}

TEST_CASE("a ray originates inside a sphere", "[rays]" ) {
    auto r = Ray{point(0, 0, 0), vector(0, 0, 1)};
    auto s = Sphere{};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0] == -1.0f);
    REQUIRE(xs[1] == 1.0f);
}

TEST_CASE("a sphere is behind a ray", "[rays]" ) {
    auto r = Ray{point(0, 0, 5), vector(0, 0, 1)};
    auto s = Sphere{};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0] == -6.0f);
    REQUIRE(xs[1] == -4.0f);
}
