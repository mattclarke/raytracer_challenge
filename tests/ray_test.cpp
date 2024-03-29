#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "transformations.h"
#include "ray.h"
#include "tuple.h"

TEST_CASE("creating and querying a ray", "[ray]" ) {
    auto origin = point(1, 2, 3);
    auto direction = vector(4, 5, 6);
    auto r = Ray{origin, direction};

    REQUIRE(r.origin == origin);
    REQUIRE(r.direction == direction);
}

TEST_CASE("computing a point from a distance", "[ray]" ) {
    auto r = Ray{point(2, 3, 4), vector(1, 0, 0)};

    REQUIRE(position(r, 0) == point(2, 3, 4));
    REQUIRE(position(r, 1) == point(3, 3, 4));
    REQUIRE(position(r, -1) == point(1, 3, 4));
    REQUIRE(position(r, 2.5) == point(4.5, 3, 4));
}

TEST_CASE("translating a ray", "[ray]" ) {
    auto r = Ray{point(1, 2, 3), vector(0, 1, 0)};
    auto m = translation(3, 4, 5);

    auto rt = transform(r, m);
    
    REQUIRE(rt.origin == point(4, 6, 8));
    REQUIRE(rt.direction == vector(0, 1, 0));
}

TEST_CASE("scaling a ray", "[ray]" ) {
    auto r = Ray{point(1, 2, 3), vector(0, 1, 0)};
    auto m = scaling(2, 3, 4);

    auto rt = transform(r, m);
    
    REQUIRE(rt.origin == point(2, 6, 12));
    REQUIRE(rt.direction == vector(0, 3, 0));
}
