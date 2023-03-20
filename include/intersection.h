#ifndef RAYTRACER_CHALLENGE_INTERSECTION_H
#define RAYTRACER_CHALLENGE_INTERSECTION_H

#include <cmath>
#include <vector>

#include "sphere.h"
#include "tuple.h"
#include "utils.h"

struct Intersection {
    float t;
    Sphere object;

    Intersection(float t, const Sphere &object) :
        t(t), object(object) 
    {}
};

std::vector<Intersection> intersections(const Intersection &i1, const Intersection &i2) {
    return {i1, i2};
}

#endif // RAYTRACER_CHALLENGE_INTERSECTION_H
