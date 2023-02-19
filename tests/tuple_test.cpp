#include <catch2/catch_test_macros.hpp>

struct Tuple {
    double x;
    double y;
    double z;
    double w;
};


TEST_CASE("A tuple with w=1.0 is a point", "[tuple]" ) {
    Tuple tuple{4.3, -4.2, 3.1, 1.0};

    REQUIRE(tuple.x == 4.3);
    REQUIRE(tuple.y == 4.3);
}