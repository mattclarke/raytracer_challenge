#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>


TEST_CASE("creating a world", "[world]" ) {
    auto w = World{10, 20};

    REQUIRE(w.objects.length() == 0);
    REQUIRE(w.lightsource == NULL);
}
