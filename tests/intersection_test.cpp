#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "intersection.h"
#include "ray.h"
#include "sphere.h"
#include "tuple.h"

TEST_CASE("an intersection encapsulates t and object", "[intersection]" ) {
    auto s = Sphere{1};

    auto i = Intersection{3.5, s};

    REQUIRE(i.t == 3.5);
    REQUIRE(i.object == s);
}
