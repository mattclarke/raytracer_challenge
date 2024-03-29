#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include "intersection.h"
#include "ray.h"
#include "sphere.h"

TEST_CASE("an intersection encapsulates t and object", "[intersection]" ) {
    auto s = Sphere{1};

    auto i = Intersection{3.5, s};

    REQUIRE(i.t == 3.5);
    REQUIRE(i.object == s);
}

TEST_CASE("aggreating intersections", "[intersection]" ) {
    auto s = Sphere{1};
    auto i1 = Intersection{1, s};
    auto i2 = Intersection{2, s};
    auto xs = intersections({i2, i1});

    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == 1.0f);
    REQUIRE(xs[1].t == 2.0f);
}

TEST_CASE("the hit, when all intersections have a positive t", "[intersection]" ) {
    auto s = Sphere{1};
    auto i1 = Intersection{1, s};
    auto i2 = Intersection{2, s};
    auto xs = intersections({i2, i1});

    auto i = hit(xs);

    REQUIRE(i == i1);
}

TEST_CASE("the hit, when some intersections have negative t", "[intersection]" ) {
    auto s = Sphere{1};
    auto i1 = Intersection{-1, s};
    auto i2 = Intersection{2, s};
    auto xs = intersections({i2, i1});

    auto i = hit(xs);

    REQUIRE(i == i2);
}

TEST_CASE("the hit, when all intersections have negative t", "[intersection]" ) {
    auto s = Sphere{1};
    auto i1 = Intersection{-1, s};
    auto i2 = Intersection{-2, s};
    auto xs = intersections({i2, i1});

    auto i = hit(xs);

    REQUIRE(!i.has_value());
}

TEST_CASE("the hit is always the lowest non-negative intersection", "[intersection]" ) {
    auto s = Sphere{1};
    auto i1 = Intersection{5, s};
    auto i2 = Intersection{2, s};
    auto i3 = Intersection{-3, s};
    auto i4 = Intersection{2, s};
    auto xs = intersections({i1, i2, i3, i4});

    auto i = hit(xs);

    REQUIRE(i == i4);
}

TEST_CASE("precomputng the state of an intersection", "[intersection]" ) {
    auto ray = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};
    auto i = Intersection{4, s};

    auto comps = prepare_computations(i, ray);

    REQUIRE(comps.t == i.t);
    REQUIRE(comps.object == i.object);
    REQUIRE(comps.point == point(0, 0, -1));
    REQUIRE(comps.eyev == vector(0, 0, -1));
    REQUIRE(comps.normalv == vector(0, 0, -1));
}

TEST_CASE("the hit, when an intersection occurs on the outside", "[intersection]" ) {
    auto ray = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};
    auto i = Intersection{4, s};

    auto comps = prepare_computations(i, ray);

    REQUIRE(comps.inside == false);
}

TEST_CASE("the hit, when an intersection occurs on the inside", "[intersection]" ) {
    auto ray = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};
    auto i = Intersection{4, s};

    auto comps = prepare_computations(i, ray);

    REQUIRE(comps.inside == true);
    REQUIRE(comps.point == point(0, 0, 1));
    REQUIRE(comps.eyev == vector(0, 0, -1));
    REQUIRE(comps.normalv == vector(0, 0, -1));
}
