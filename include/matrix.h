#ifndef RAYTRACER_CHALLENGE_MATRIX_H
#define RAYTRACER_CHALLENGE_MATRIX_H

#include <vector>

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
    // TODO: Assert sizes are the same and square
    std::vector<float> calculated(lhs.width * lhs.height, 0);

    for (size_t r = 0; r < lhs.width; ++r) {
        for (size_t c = 0; c < lhs.height; ++c) {
            float ans = 0;
            for (size_t i = 0; i < lhs.width; ++i) {
                ans += lhs.at(r, i) * rhs.at(i, c);
            }
            calculated[r * lhs.width + c] = ans;
        }
    }
    return {lhs.width, lhs.height, calculated};
}

#endif //RAYTRACER_CHALLENGE_MATRIX_H
