#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "sphere.h"
#include "tuple.h"

TEST_CASE("the default material", "[material]" ) {
    auto m = Material{};
    m.colour = Colour{1, 1, 1};

    REQUIRE(m.ambient == 0.1f);
    REQUIRE(m.diffuse == 0.9f);
    REQUIRE(m.specular == 0.9f);
    REQUIRE(m.shininess == 200.0f);
}
