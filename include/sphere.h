#ifndef RAYTRACER_CHALLENGE_SPHERE_H
#define RAYTRACER_CHALLENGE_SPHERE_H

#include <cmath>

#include "tuple.h"
#include "utils.h"

struct Sphere {
    int id;

    explicit Sphere(int id) : id(id) {
    }
};

bool operator==(const Sphere &lhs, const Sphere &rhs){
    return lhs.id == rhs.id;
}

#endif // RAYTRACER_CHALLENGE_SPHERE_H
