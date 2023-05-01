#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>

#include "world.h"


TEST_CASE("creating a world", "[world]" ) {
    auto w = World{};

    REQUIRE(w.objects.size() == 0);
    REQUIRE(w.lightsource == NULL);
}
