#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>

#include "canvas.h"
#include "tuple.h"


std::vector<std::string> str_to_vec(const std::string &s) {
    std::vector<std::string> result; 
    std::istringstream iss;
    iss.str(s);
    for (std::string temp; std::getline(iss, temp, '\n');) {
        result.push_back(temp);
    }
    return result;
}

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

TEST_CASE("constructing the PPM header", "[canvas]" ) {
    auto c = Canvas{5, 3};

    auto ppm = canvas_to_ppm(c);
    auto lines = str_to_vec(ppm);


    REQUIRE(lines[0] == "P3");
    REQUIRE(lines[1] == "5 3");
    REQUIRE(lines[2] == "255");
}

TEST_CASE("constructing the PPM pixel data", "[canvas]" ) {
    auto canvas = Canvas{5, 3};
    auto c1 = Colour{1.5, 0, 0};
    auto c2 = Colour{0, 0.5, 0};
    auto c3 = Colour{-0.5, 0, 1};

    write_pixel(canvas, 0, 0, c1);
    write_pixel(canvas, 2, 1, c2);
    write_pixel(canvas, 4, 2, c3);

    auto ppm = canvas_to_ppm(canvas);
    auto lines = str_to_vec(ppm);

    REQUIRE(lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    REQUIRE(lines[4] == "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    REQUIRE(lines[5] == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

TEST_CASE("splitting long lines in PPM files", "[canvas]" ) {
    auto canvas = Canvas{10, 2};
    auto c1 = Colour{1, 0.8, 0.6};

    for (size_t y = 0; y < 2; ++y) {
        for (size_t x = 0; x < 10; ++x) {
            write_pixel(canvas, x, y, c1);
        }
    }

    auto ppm = canvas_to_ppm(canvas);
    auto lines = str_to_vec(ppm);

    REQUIRE(lines[3] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    REQUIRE(lines[4] == "153 255 204 153 255 204 153 255 204 153 255 204 153");
    REQUIRE(lines[5] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204");
    REQUIRE(lines[6] == "153 255 204 153 255 204 153 255 204 153 255 204 153");
}
