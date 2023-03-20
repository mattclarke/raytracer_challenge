#ifndef RAYTRACER_CHALLENGE_SPHERE_H
#define RAYTRACER_CHALLENGE_SPHERE_H

#include <cmath>

#include "matrix.h"
#include "tuple.h"
#include "utils.h"

struct Sphere {
    int id;
    Matrix transform = {4, 4, {1, 0, 0, 0,
                               0, 1, 0, 0,
                               0, 0, 1, 0,
                               0, 0, 0, 1}};

    explicit Sphere(int id) : id(id) {
    }
};

bool operator==(const Sphere &lhs, const Sphere &rhs){
    return lhs.id == rhs.id;
}

void set_transform(Sphere &s, Matrix m) {
    s.transform = m;
}

#endif // RAYTRACER_CHALLENGE_SPHERE_H
