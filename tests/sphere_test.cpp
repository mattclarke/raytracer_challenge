#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "ray.h"
#include "sphere.h"
#include "tuple.h"

TEST_CASE("spheres are unique", "[sphere]" ) {
    auto s1 = Sphere{1};
    auto s2 = Sphere{2};

    REQUIRE(s1 != s2);
    REQUIRE(s1 == s1);
    REQUIRE(s2 == s2);
}

TEST_CASE("a ray intersects a sphere at two points", "[sphere]" ) {
    auto r = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == 4.0f);
    REQUIRE(xs[1].t == 6.0f);
}

TEST_CASE("a ray intersects a sphere at a tangent", "[sphere]" ) {
    auto r = Ray{point(0, 1, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == 5.0f);
    REQUIRE(xs[1].t == 5.0f);
}

TEST_CASE("a ray misses a sphere", "[sphere]" ) {
    auto r = Ray{point(0, 2, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.empty());
}

TEST_CASE("a ray originates inside a sphere", "[sphere]" ) {
    auto r = Ray{point(0, 0, 0), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == -1.0f);
    REQUIRE(xs[1].t == 1.0f);
}

TEST_CASE("a sphere is behind a ray", "[sphere]" ) {
    auto r = Ray{point(0, 0, 5), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == -6.0f);
    REQUIRE(xs[1].t == -4.0f);
}

TEST_CASE("intersect sets the object on the intersection", "[sphere]" ) {
    auto r = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    auto xs = intersect(s, r);

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].object == s);
    REQUIRE(xs[1].object == s);
}
