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
