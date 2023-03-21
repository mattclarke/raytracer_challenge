#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "light.h"
#include "tuple.h"

TEST_CASE("a point light has a position and intensity", "[light]" ) {
    auto intensity = Colour{1, 1, 1};
    auto position = point(0, 0, 0);

    auto light = PointLight{position, intensity};

    REQUIRE(light.position == position);
    REQUIRE(light.intensity == intensity);
}
