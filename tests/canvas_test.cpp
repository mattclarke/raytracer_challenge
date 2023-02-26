#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include "canvas.h"
#include "tuple.h"

TEST_CASE("creating a blank canvas", "[canvas]" ) {
    auto c = Canvas{10, 20};

    REQUIRE(c.width == 10);
    REQUIRE(c.height == 20);
    for (const auto &pixel : c.pixels) {
        REQUIRE(pixel == Colour{0, 0, 0});
    }
}

TEST_CASE("writing pixels to a canvas", "[canvas]" ) {
    auto c = Canvas{10, 20};
    auto red = Colour{1, 0, 0};

    write_pixel(c, 2, 3, red);

    REQUIRE(pixel_at(c, 2, 3) == red);
}
