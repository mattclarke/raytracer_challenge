#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_floating_point.hpp>

#include <string>

#include "matrix.h"
#include "tuple.h"


TEST_CASE("constructing and inspecting a 4x4 matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {1, 2, 3, 4, 5.5, 6.5, 7.5, 8.5, 9, 10, 11, 12, 13.5, 14.5, 15.5, 16.5}};

    REQUIRE(m.at(0, 0) == 1);
    REQUIRE(m.at(0, 3) == 4);
    REQUIRE(m.at(1, 0) == 5.5);
    REQUIRE(m.at(1, 2) == 7.5);
    REQUIRE(m.at(2, 2) == 11);
    REQUIRE(m.at(3, 0) == 13.5);
    REQUIRE(m.at(3, 2) == 15.5);
}

TEST_CASE("a 2x2 matrix ought to be representable", "[matrix]" ) {
    auto m = Matrix{2, 2, {-3, 5, 1, -2}};

    REQUIRE(m.at(0, 0) == -3);
    REQUIRE(m.at(0, 1) == 5);
    REQUIRE(m.at(1, 0) == 1);
    REQUIRE(m.at(1, 1) == -2);
}

TEST_CASE("a 3x3 matrix ought to be representable", "[matrix]" ) {
    auto m = Matrix{3, 3, {-3, 5, 0, 1, -2, -7, 0, 1, 1}};

    REQUIRE(m.at(0, 0) == -3);
    REQUIRE(m.at(1, 1) == -2);
    REQUIRE(m.at(2, 2) == 1);
}

TEST_CASE("matrix equality with identical matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};

    REQUIRE(m1 == m2);
}

TEST_CASE("matrix equality with different matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1}};

    REQUIRE(m1 != m2);
}

TEST_CASE("multiplying two matrices", "[matrix]" ) {
    auto m1 = Matrix{4, 4, {1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2}};
    auto m2 = Matrix{4, 4, {-2, 1, 2, 3, 3, 2, 1, -1, 4, 3, 6, 5, 1, 2, 7, 8}};

    auto expected = Matrix{4, 4, {20, 22, 50, 48, 44, 54, 114, 108, 40, 58, 110, 102, 16, 26, 46, 42}};

    REQUIRE(m1 * m2 == expected);
}

TEST_CASE("matrix multiplied by a tuple", "[matrix]" ) {
    auto m = Matrix{4, 4, {1, 2, 3, 4, 2, 4, 4, 2, 8, 6, 4, 1, 0, 0, 0, 1}};
    auto t = Tuple{1, 2, 3, 1};

    REQUIRE(m * t == Tuple{18, 24, 33, 1});
}

TEST_CASE("muliplying a matrix by the identity matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {0, 1, 2, 4, 1, 2, 4, 8, 2, 4, 8, 16, 4, 8, 16, 32}};
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};

    REQUIRE(m * idm == m);
}

TEST_CASE("multiplying the identity matrix by a tuple", "[matrix]" ) {
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};
    auto t = Tuple{1, 2, 3, 4};

    REQUIRE(idm * t == t);
}

TEST_CASE("transposing a matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {0, 9, 3, 0,
                           9, 8, 0, 8,
                           1, 8, 5, 3,
                           0, 0, 5, 8}};

    auto expected = Matrix{4, 4, {0, 9, 1, 0,
                                  9, 8, 8, 0,
                                  3, 0, 5, 5,
                                  0, 8, 3, 8}};
    REQUIRE(transpose(m) == expected);
}

TEST_CASE("transposing the identity matrix", "[matrix]" ) {
    auto idm = Matrix{4, 4, {1, 0, 0, 0,
                             0, 1, 0, 0,
                             0, 0, 1, 0,
                             0, 0, 0, 1}};

    REQUIRE(transpose(idm) == idm);
}

TEST_CASE("calculating the determinant of a 2x2 matrix", "[matrix]" ) {
    auto m = Matrix{2, 2, {1, 5, -3, 2}};

    REQUIRE(determinant(m) == 17);
}

TEST_CASE("a submatrix of a 3x3 matrix is a 2x2 matrix", "[matrix]" ) {
    auto m = Matrix{3, 3, {1, 5, 0, 3, 2, 7, 0, 6, -3}};

    auto expected = Matrix{2, 2, {3, 2, 0, 6}};

    REQUIRE(submatrix(m, 0, 2) == expected);
}

TEST_CASE("a submatrix of a 4x4 matrix is a 3x3 matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {-6, 1, 1, 6, -8, 5, 8, 6, -1, 0, 8, 2, -7, 1, -1, 1}};

    auto expected = Matrix{3, 3, {-6, 1, 6, -8, 8, 6, -7, -1, 1}};

    REQUIRE(submatrix(m, 2, 1) == expected);
}

TEST_CASE("calculating a minor of a 3x3 matrix", "[matrix]" ) {
    auto m = Matrix{3, 3, {3, 5, 0, 2, -1, -7, 6, -1, 5}};

    REQUIRE(minor(m, 1, 0) == 25);
    REQUIRE(determinant(submatrix(m, 1, 0)) == minor(m, 1, 0));
}

TEST_CASE("calculating a cofactor of a 3x3 matrix", "[matrix]" ) {
    auto m = Matrix{3, 3, {3, 5, 0, 2, -1, -7, 6, -1, 5}};

    REQUIRE(minor(m, 0, 0) == -12);
    REQUIRE(cofactor(m, 0, 0) == -12);
    REQUIRE(minor(m, 1, 0) == 25);
    REQUIRE(cofactor(m, 1, 0) == -25);
}

TEST_CASE("calculating the determinant of a 3x3 matrix", "[matrix]" ) {
    auto m = Matrix{3, 3, {1, 2, 6, -5, 8, -4, 2, 6, 4}};

    REQUIRE(cofactor(m, 0, 0) == 56);
    REQUIRE(cofactor(m, 0, 1) == 12);
    REQUIRE(cofactor(m, 0, 2) == -46);
    REQUIRE(determinant(m) == -196);
}

TEST_CASE("calculating the determinant of a 4x4 matrix", "[matrix]" ) {
    auto m = Matrix{4, 4, {-2, -8, 3, 5, -3, 1, 7, 3, 1, 2, -9, 6, -6, 7, 7, -9}};

    REQUIRE(cofactor(m, 0, 0) == 690);
    REQUIRE(cofactor(m, 0, 1) == 447);
    REQUIRE(cofactor(m, 0, 2) == 210);
    REQUIRE(cofactor(m, 0, 3) == 51);
    REQUIRE(determinant(m) == -4071);
}

TEST_CASE("testing an invertible matrix for invertibility", "[matrix]" ) {
    auto m = Matrix{4, 4, {6, 4, 4, 4, 5, 5, 7, 6, 4, -9, 3, -7, 9, 1, 7, -6}};

    REQUIRE(determinant(m) == -2120);
    REQUIRE(invertible(m));
}

TEST_CASE("testing a noninvertible matrix for invertibility", "[matrix]" ) {
    auto m = Matrix{4, 4, {-4, 2, -2, -3, 9, 6, 2, 6, 0, -5, 1, -5, 0, 0, 0, 0}};

    REQUIRE(determinant(m) == 0);
    REQUIRE(!invertible(m));
}

TEST_CASE("calculating the inverse of a matrix", "[matrix]" ) {
    auto a = Matrix{4, 4, {-5, 2, 6, -8, 1, -5, 1, 8, 7, 7, -6, -7, 1, -3, 7, 4}};

    auto b = inverse(a);

    REQUIRE(determinant(a) == 532);
    REQUIRE(cofactor(a, 2, 3) == -160);
    REQUIRE(b.at(3, 2) == -160.0f/532.0f);
    REQUIRE(cofactor(a, 3, 2) == 105);
    REQUIRE(b.at(2, 3) == 105.0f/532.0f);
    REQUIRE(b == Matrix{4, 4, {0.218045f, 0.451128f, 0.240602f, -0.0451128f, -0.808271f, -1.45677f, -0.443609f, 0.520677f, -0.0789474f, -0.223684f, -0.0526316f, 0.197368f, -0.522556f, -0.81391f, -0.300752f, 0.306391f}});
}

TEST_CASE("checking the cofactors of a matrix", "[matrix]" ) {
    auto a = Matrix{4, 4, {-5, 2, 6, -8, 1, -5, 1, 8, 7, 7, -6, -7, 1, -3, 7, 4}};

    REQUIRE(cofactor(a, 0, 0) == 116.0f);
    REQUIRE(cofactor(a, 0, 1) == -430.0f);
    REQUIRE(cofactor(a, 0, 2) == -42.0f);
    REQUIRE(cofactor(a, 0, 3) == -278.0f);
    
    REQUIRE(cofactor(a, 1, 0) == 240.0f);
    REQUIRE(cofactor(a, 1, 1) == -775.0f);
    REQUIRE(cofactor(a, 1, 2) == -119.0f);
    REQUIRE(cofactor(a, 1, 3) == -433.0f);
}

TEST_CASE("calculating the inverse of another matrix", "[matrix]" ) {
    auto a = Matrix{4, 4, {8, -5, 9, 2, 7, 5, 6, 1, -6, 0, 9, 6, -3, 0, -9, -4}};

    auto b = inverse(a);

    REQUIRE(b == Matrix{4, 4, {-0.15385f, -0.15385f, -0.28205f, -0.53846f, -0.07692f, 0.12308f, 0.02564f, 0.03077f, 0.35897f, 0.35897f, 0.43590f, 0.92308f, -0.69231f, -0.69231f, -0.76923f, -1.92308f}});
}

TEST_CASE("calculating the inverse of yet another matrix", "[matrix]" ) {
    auto a = Matrix{4, 4, {9, 3, 0, 9, -5, -2, -6, -3, -4, 9, 6, 4, -7, 6, 6, 2}};

    auto b = inverse(a);

    REQUIRE(b == Matrix{4, 4, {-0.04074f, -0.07778f, 0.14444f, -0.22222f, -0.07778f, 0.03333f, 0.36667f, -0.33333f, -0.02901f, -0.14630f, -0.10926f, 0.12963f, 0.17778f, 0.06667f, -0.26667f, 0.33333f}});
}

TEST_CASE("multiplying a product by its inverse", "[matrix]" ) {
    auto a = Matrix{4, 4, {3, -9, 7, 3, 3, -8, 2, -9, -4, 4, 4, 1, -6, 5, -1, 1}};
    auto b = Matrix{4, 4, {8, 2, 2, 2, 3, -1, 7, 0, 7, 0, 5, 4, 6, -2, 0, 5}};

    auto c = a * b;

    REQUIRE(c * inverse(b) == a);
}
