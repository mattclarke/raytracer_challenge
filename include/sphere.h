#ifndef RAYTRACER_CHALLENGE_SPHERE_H
#define RAYTRACER_CHALLENGE_SPHERE_H

#include <cmath>

#include "material.h"
#include "matrix.h"
#include "tuple.h"
#include "utils.h"

struct Sphere {
    int id;
    Matrix transform = {4, 4, {1, 0, 0, 0,
                               0, 1, 0, 0,
                               0, 0, 1, 0,
                               0, 0, 0, 1}};
    Material material = {};

    explicit Sphere(int id) : id(id) {
    }
};

bool operator==(const Sphere &lhs, const Sphere &rhs){
    return lhs.id == rhs.id;
}

void set_transform(Sphere &s, Matrix m) {
    s.transform = std::move(m);
}

Tuple normal_at(const Sphere &s, const Tuple &wp) {
    auto object_point = inverse(s.transform) * wp;
    auto object_normal = object_point - point(0, 0, 0);
    auto world_normal = transpose(inverse(s.transform)) * object_normal;
    world_normal.w = 0;
    return normalise(world_normal);
}

#endif // RAYTRACER_CHALLENGE_SPHERE_H
