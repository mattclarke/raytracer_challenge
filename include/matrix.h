#ifndef RAYTRACER_CHALLENGE_MATRIX_H
#define RAYTRACER_CHALLENGE_MATRIX_H

#include <vector>

struct Matrix {
    Matrix(size_t width, size_t height, const std::vector<float> &values)
        : width(width), height(height), values(values) {
        // TODO: assert if values are the wrong size
    }

    float at(size_t y, size_t x) {
        return values[y * width + x];
    }

    size_t width;
    size_t height;
    std::vector<float> values;
};

#endif //RAYTRACER_CHALLENGE_MATRIX_H
