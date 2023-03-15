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

    for (size_t r = 0; r < lhs.height; ++r) {
        for (size_t c = 0; c < lhs.width; ++c) {
            if (lhs.at(r, c) != rhs.at(r,c)) {
                return false;
            }
        }
    }
    return true;
}
#endif //RAYTRACER_CHALLENGE_MATRIX_H
