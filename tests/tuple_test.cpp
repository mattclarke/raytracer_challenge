#include <catch2/catch_test_macros.hpp>

struct Tuple {
    double x;
    double y;
    double z;
    double w;

    bool is_point() {
        return w == 1.0;
    }

    bool is_vector() {
        return w == 0.0;
    }

};

bool operator==(const Tuple &lhs, const Tuple &rhs) {
    // TODO: use epsilon for comparison?
    return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z && lhs.w == rhs.w;
}

Tuple operator+(const Tuple &lhs, const Tuple &rhs) {
    return {lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w};
}

Tuple operator-(const Tuple &lhs, const Tuple &rhs) {
    return {lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w};
}

Tuple operator-(const Tuple &rhs) {
    return {-rhs.x, -rhs.y, -rhs.z, -rhs.w};
}

Tuple operator*(const Tuple &tuple, double scalar) {
    return {tuple.x * scalar, tuple.y * scalar, tuple.z * scalar, tuple.w * scalar};
}

Tuple operator/(const Tuple &tuple, double scalar) {
    return {tuple.x / scalar, tuple.y / scalar, tuple.z / scalar, tuple.w / scalar};
}

Tuple point(double x, double y, double z) {
    return {x, y, z, 1.0};
}

Tuple vector(double x, double y, double z) {
    return {x, y, z, 0.0};
}

TEST_CASE("a tuple with w=1 is a point", "[tuple]" ) {
    Tuple tuple{4.3, -4.2, 3.1, 1.0};

    REQUIRE(tuple.x == 4.3);
    REQUIRE(tuple.y == -4.2);
    REQUIRE(tuple.z == 3.1);
    REQUIRE(tuple.w == 1.0);
    REQUIRE(tuple.is_point() == true);
    REQUIRE(tuple.is_vector() == false);
}

TEST_CASE("a tuple with w=0 is a vector", "[tuple]" ) {
    Tuple tuple{4.3, -4.2, 3.1, 0.0};

    REQUIRE(tuple.x == 4.3);
    REQUIRE(tuple.y == -4.2);
    REQUIRE(tuple.z == 3.1);
    REQUIRE(tuple.w == 0.0);
    REQUIRE(tuple.is_point() == false);
    REQUIRE(tuple.is_vector() == true);
}

TEST_CASE("point() creates tuple with w=1", "[tuple]" ) {
    Tuple tuple = point(4, -4, 3);

    REQUIRE(tuple == Tuple{4, -4, 3, 1.0});
}

TEST_CASE("vector() creates vector with w=0", "[tuple]" ) {
    Tuple tuple = vector(4, -4, 3);

    REQUIRE(tuple == Tuple{4, -4, 3, 0.0});
}

TEST_CASE("adding two tuples", "[tuple]" ) {
    Tuple t1{3, -2, 5, 1};
    Tuple t2{-2, 3, 1, 0};

    REQUIRE(t1 + t2 == Tuple{1, 1, 6, 1.0});
}

TEST_CASE("subtracting two points", "[tuple]" ) {
    auto pt1 = point(3, 2, 1);
    auto pt2 = point(5, 6, 7);

    REQUIRE(pt1 - pt2 == vector(-2, -4, -6));
}

TEST_CASE("subtracting a vector from a point", "[tuple]" ) {
    auto pt = point(3, 2, 1);
    auto vec = vector(5, 6, 7);

    REQUIRE(pt - vec == point(-2, -4, -6));
}

TEST_CASE("subtracting two vectors", "[tuple]" ) {
    auto vec1 = vector(3, 2, 1);
    auto vec2 = vector(5, 6, 7);

    REQUIRE(vec1 - vec2 == vector(-2, -4, -6));
}

TEST_CASE("subtracting a vector from the zero vector", "[tuple]" ) {
    auto zero = vector(0, 0, 0);
    auto vec = vector(1, -2, 3);

    REQUIRE(zero - vec == vector(-1, 2, -3));
}

TEST_CASE("negating a tuple", "[tuple]" ) {
    Tuple tuple = Tuple{1, -2, 3, -4};

    REQUIRE(-tuple == Tuple{-1, 2, -3, 4});
}

TEST_CASE("multiplying a tuple by a scalar", "[tuple]" ) {
    Tuple tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple * 3.5 == Tuple{3.5, -7, 10.5, -14});
}

TEST_CASE("multiplying a tuple by a fraction", "[tuple]" ) {
    Tuple tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple * 0.5 == Tuple{0.5, -1, 1.5, -2});
}

TEST_CASE("dividing a tuple by a scalar", "[tuple]" ) {
    Tuple tuple = Tuple{1, -2, 3, -4};

    REQUIRE(tuple / 2 == Tuple{0.5, -1, 1.5, -2});
}
