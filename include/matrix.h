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

    [[nodiscard]] float at(size_t row, size_t column) const {
        return values[row * width + column];
    }

    size_t width;
    size_t height;
    std::vector<float> values;
};

bool operator==(const Matrix &lhs, const Matrix &rhs) {
    if (lhs.width != rhs.width || lhs.height != rhs.height) {
        return false;
    }

    for (size_t i = 0; i < lhs.values.size(); ++i) {
        if (!equal(lhs.values[i], rhs.values[i])){
            return false;
        }
    }
    
    return true;
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

Matrix transpose(const Matrix &m) {
    assert((void("matrix must be 4x4 when transposing"), m.width == 4 && m.height == 4));

    std::vector<float> values;

    for (size_t c = 0; c < 4; ++c) {
        for (size_t r = 0; r < 4; ++r) {
            values.push_back(m.at(r, c));
        }
    }
    return {4, 4, values};
}

float cofactor(const Matrix &m, size_t row, size_t column);

float determinant(const Matrix &m) {
    if (m.width == 2) {
        return m.at(0, 0) * m.at(1, 1) - m.at(0, 1) * m.at(1, 0);
    }
    
    auto result = 0.0f;
    for (size_t c = 0; c < m.width; ++c) {
        result = result + m.at(0, c) * cofactor(m, 0, c);
    }
    return result;
}

Matrix submatrix(const Matrix &m, size_t row, size_t column) {
    std::vector<float> values;

    for (size_t r = 0; r < m.width; ++r) {
        for (size_t c = 0; c < m.height; ++c) {
            if (r != row && c != column) {
                values.push_back(m.at(r, c));
            }
        }
    }
    return {m.width - 1, m.height - 1, values};
}

float minor(const Matrix &m, size_t row, size_t column) {
    return determinant(submatrix(m, row, column));
}

float cofactor(const Matrix &m, size_t row, size_t column) {
    auto result = minor(m, row, column);

    return ((row + column) % 2 == 0) ? result : -result;
}

bool invertible(const Matrix &m) {
    return determinant(m) != 0;
}

Matrix inverse(const Matrix &m) {
    assert((void("matrices must be invertible to be inversed"), invertible(m)));

    std::vector values(m.width * m.height, 0.0f);

    for (size_t r = 0; r < m.width; ++r) {
        for (size_t c = 0; c < m.height; ++c) {
            std::cout << cofactor(m, r, c) << ' ';
            // Note: c and r are swapped so it does a transpose for free!
            values[c * m.width + r] = cofactor(m, r, c) / determinant(m);
        }
    }
    return {m.width, m.height, values};
}

#endif //RAYTRACER_CHALLENGE_MATRIX_H
