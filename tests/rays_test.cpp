#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "tuple.h"

TEST_CASE("creating and querying a ray", "[rays]" ) {
    auto origin = point(1, 2, 3);
    auto direction = vector(4, 5, 6);
    auto r = Ray{origin, direction};

    REQUIRE(r.origin == origin);
    REQUIRE(r.direction == direction);
}
