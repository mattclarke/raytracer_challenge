#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>
#include <iostream>
#include <sstream>
#include <string>

#include "world.h"
#include "light.h"
#include "tuple.h"
#include "sphere.h"
#include "transformations.h"

template<typename T>
bool contains(T item, std::vector<T> vec) {
    return std::find(vec.begin(), vec.end(), item) != vec.end();
}


TEST_CASE("creating a world", "[world]" ) {
    auto w = World{};

    REQUIRE(w.objects.size() == 0);
    REQUIRE(w.lightsource == nullptr);
}

TEST_CASE("the default world", "[world]" ) {
    auto sphere1 = Sphere{1};
    sphere1.material = Material{};
    sphere1.material.color = Color{0.8, 1.0, 0.6};
    sphere1.material.diffuse = 0.7f;
    sphere1.material.specular = 0.2f;
    auto sphere2 = Sphere{2};
    set_transform(sphere2, scaling(0.5, 0.5, 0.5));

    auto w = create_default_world();

    REQUIRE(w->objects.size() == 2);
    REQUIRE(contains(sphere1, w->objects) == true);
    REQUIRE(contains(sphere2, w->objects) == true);
    REQUIRE(w->lightsource->position == point(0, 0, 0));
    REQUIRE(w->lightsource->intensity == Color{1, 1, 1});
}


