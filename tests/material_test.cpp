#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "light.h"
#include "material.h"
#include "tuple.h"

TEST_CASE("the default material", "[material]" ) {
    auto m = Material{};
    m.colour = Colour{1, 1, 1};

    REQUIRE(m.ambient == 0.1f);
    REQUIRE(m.diffuse == 0.9f);
    REQUIRE(m.specular == 0.9f);
    REQUIRE(m.shininess == 200.0f);
}

TEST_CASE("lighting with the eye between the light and the surface", "[material]" ) {
    auto m = Material{};
    auto position = point(0, 0, 0);

    auto eyev = vector(0, 0, -1);
    auto normalv = vector(0, 0, -1);
    auto light = PointLight{point(0, 0, -10), Colour{1, 1, 1}};

    auto result = lighting(m, light, position, eyev, normalv);

    REQUIRE(result == Colour{1.9, 1.9, 1.9});
}

TEST_CASE("lighting with the eye between the light and the surface, eye offset 45 degrees", "[material]" ) {
    auto m = Material{};
    auto position = point(0, 0, 0);

    auto eyev = vector(0, std::sqrt(2) / 2, -std::sqrt(2) / 2);
    auto normalv = vector(0, 0, -1);
    auto light = PointLight{point(0, 0, -10), Colour{1, 1, 1}};

    auto result = lighting(m, light, position, eyev, normalv);

    REQUIRE(result == Colour{1.0, 1.0, 1.0});
}

TEST_CASE("lighting with eye opposite surface, light offset 45 degrees", "[material]" ) {
    auto m = Material{};
    auto position = point(0, 0, 0);

    auto eyev = vector(0, 0, -1);
    auto normalv = vector(0, 0, -1);
    auto light = PointLight{point(0, 10, -10), Colour{1, 1, 1}};

    auto result = lighting(m, light, position, eyev, normalv);

    REQUIRE(result == Colour{0.7364, 0.7364, 0.7364});
}

TEST_CASE("lighting with the eye in the path of the reflection vector", "[material]" ) {
    auto m = Material{};
    auto position = point(0, 0, 0);

    auto eyev = vector(0, -std::sqrt(2) / 2, -std::sqrt(2) / 2);
    auto normalv = vector(0, 0, -1);
    auto light = PointLight{point(0, 10, -10), Colour{1, 1, 1}};

    auto result = lighting(m, light, position, eyev, normalv);

    REQUIRE(result == Colour{1.6364, 1.6364, 1.6364});
}

TEST_CASE("lighting with the light behind the surface", "[material]" ) {
    auto m = Material{};
    auto position = point(0, 0, 0);

    auto eyev = vector(0, 0, -1);
    auto normalv = vector(0, 0, -1);
    auto light = PointLight{point(0, 0, 10), Colour{1, 1, 1}};

    auto result = lighting(m, light, position, eyev, normalv);

    REQUIRE(result == Colour{0.1, 0.1, 0.1});
}
