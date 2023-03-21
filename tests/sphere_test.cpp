#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <cmath>

#include "transformations.h"
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

TEST_CASE("a sphere's default transformation", "[sphere]" ) {
    auto s = Sphere{1};
    
    auto idm = Matrix{4, 4, {1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1}};

    REQUIRE(s.transform == idm);
}

TEST_CASE("changing a sphere's transformation", "[sphere]" ) {
    auto s = Sphere{1};
    auto t = translation(2, 3, 4);

    set_transform(s, t);
    
    REQUIRE(s.transform == t);
}

TEST_CASE("intersecting a scaled sphere with a ray", "[sphere]" ) {
    auto r = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    set_transform(s, scaling(2, 2, 2));
    auto xs = intersect(s, r);
    
    REQUIRE(xs.size() == 2);
    REQUIRE(xs[0].t == 3.0f);
    REQUIRE(xs[1].t == 7.0f);
}

TEST_CASE("intersecting a translated sphere with a ray", "[sphere]" ) {
    auto r = Ray{point(0, 0, -5), vector(0, 0, 1)};
    auto s = Sphere{1};

    set_transform(s, translation(5, 0, 0));
    auto xs = intersect(s, r);
    
    REQUIRE(xs.empty());
}

TEST_CASE("the normal on a sphere at a point on the x axis", "[sphere]" ) {
    auto s = Sphere{1};

    auto n = normal_at(s, point(1, 0, 0));
    
    REQUIRE(n == vector(1, 0, 0));
}

TEST_CASE("the normal on a sphere at a point on the y axis", "[sphere]" ) {
    auto s = Sphere{1};

    auto n = normal_at(s, point(0, 1, 0));
    
    REQUIRE(n == vector(0, 1, 0));
}

TEST_CASE("the normal on a sphere at a point on the z axis", "[sphere]" ) {
    auto s = Sphere{1};

    auto n = normal_at(s, point(0, 0, 1));
    
    REQUIRE(n == vector(0, 0, 1));
}

TEST_CASE("the normal on a sphere at a nonaxial point", "[sphere]" ) {
    auto s = Sphere{1};

    auto n = normal_at(s, point(std::sqrt(3) / 3, std::sqrt(3) / 3, std::sqrt(3) / 3));
    
    REQUIRE(n == vector(std::sqrt(3) / 3, std::sqrt(3) / 3, std::sqrt(3) / 3));
}

TEST_CASE("the normal is a normalised vector", "[sphere]" ) {
    auto s = Sphere{1};

    auto n = normal_at(s, point(std::sqrt(3) / 3, std::sqrt(3) / 3, std::sqrt(3) / 3));
    
    REQUIRE(n == normalise(n));
}
