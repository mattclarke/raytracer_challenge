#ifndef RAYTRACER_CHALLENGE_TRANSFORMATIONS_H
#define RAYTRACER_CHALLENGE_TRANSFORMATIONS_H

#include <cmath>

#include "matrix.h"

Matrix translation(float x, float y, float z) {
    return {4, 4, {1, 0, 0, x, 0, 1, 0, y, 0, 0, 1, z, 0, 0, 0, 1}};
}

Matrix scaling(float x, float y, float z) {
    return {4, 4, {x, 0, 0, 0, 0, y, 0, 0, 0, 0, z, 0, 0, 0, 0, 1}};
}

Matrix rotation_x(float r) {
    return {4, 4, {1, 0, 0, 0, 0, std::cos(r), -std::sin(r), 0, 0, std::sin(r), std::cos(r), 0, 0, 0, 0, 1}};
}

Matrix rotation_y(float r) {
    return {4, 4, {std::cos(r), 0, std::sin(r), 0, 0, 1, 0, 0, -std::sin(r), 0,  std::cos(r), 0, 0, 0, 0, 1}};
}

Matrix rotation_z(float r) {
    return {4, 4, {std::cos(r), -std::sin(r), 0, 0, std::sin(r), std::cos(r), 0, 0, 0, 0, 1, 0, 0, 0, 0, 1}};
}

Matrix shearing(float xy, float xz, float yx, float yz, float zx, float zy) {
    return {4, 4, {1, xy, xz, 0, yx, 1, yz, 0, zx, zy, 1, 0, 0, 0, 0, 1}};
}
#endif //RAYTRACER_CHALLENGE_TRANSFORMATIONS_H
