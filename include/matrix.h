#ifndef RAYTRACER_CHALLENGE_MATRIX_H
#define RAYTRACER_CHALLENGE_MATRIX_H

#include <cassert>
#include <vector>

#include "tuple.h"
#include "utils.h"

struct Matrix {
    Matrix(size_t width, size_t height, const std::vector<float> &values)
        : width(width), height(height), values(values) {
        // TODO: assert if values are the wrong size
    }

    float at(size_t y, size_t x) const {
        return values[y * width + x];
    }

    size_t width;
    size_t height;
    std::vector<float> values;
};

bool operator==(const Matrix &lhs, const Matrix &rhs) {
    if (lhs.width != rhs.width || lhs.height != rhs.height) {
        return false;
    }
    
    return lhs.values == rhs.values;
}

Matrix operator*(const Matrix &lhs, const Matrix &rhs) {
    assert((void("matrices must be 4x4 when multiplying"), lhs.width == 4 && lhs.height == 4));
    assert((void("matrices must be 4x4 when multiplying"), rhs.width == 4 && rhs.height == 4));

    std::vector<float> calculated(16, 0);

    for (size_t r = 0; r < 4; ++r) {
        for (size_t c = 0; c < 4; ++c) {
            calculated[r * 4 + c] = lhs.at(r, 0) * rhs.at(0, c)
                + lhs.at(r, 1) * rhs.at(1, c)
                + lhs.at(r, 2) * rhs.at(2, c)
                + lhs.at(r, 3) * rhs.at(3, c);
        }
    }
    return {4, 4, calculated};
}

Tuple operator*(const Matrix &lhs, const Tuple &rhs) {
    assert((void("matrix must be 4x4 when multiplying against a tuple"), lhs.width == 4 && lhs.height == 4));

    return {
        lhs.at(0, 0) * rhs.x + lhs.at(0, 1) * rhs.y
        + lhs.at(0, 2) * rhs.z + lhs.at(0, 3) * rhs.w,
        
        lhs.at(1, 0) * rhs.x + lhs.at(1, 1) * rhs.y
        + lhs.at(1, 2) * rhs.z + lhs.at(1, 3) * rhs.w,
        
        lhs.at(2, 0) * rhs.x + lhs.at(2, 1) * rhs.y
        + lhs.at(2, 2) * rhs.z + lhs.at(2, 3) * rhs.w,
        
        lhs.at(3, 0) * rhs.x + lhs.at(3, 1) * rhs.y
        + lhs.at(3, 2) * rhs.z + lhs.at(3, 3) * rhs.w
    };
}

#endif //RAYTRACER_CHALLENGE_MATRIX_H
