#ifndef RAYTRACER_CHALLENGE_RAY_H
#define RAYTRACER_CHALLENGE_RAY_H

#include <vector>

#include "tuple.h"

struct Intersection;
struct Matrix;
struct Sphere;

struct Ray {
    Tuple origin;
    Tuple direction;

    Ray(const Tuple &origin, const Tuple &direction) :
        origin(origin), direction(direction) 
    {}
};

Tuple position(const Ray &ray, float t);

Ray transform(const Ray &r, const Matrix &m);

std::vector<Intersection> intersect(const Sphere &s, const Ray &r);

#endif // RAYTRACER_CHALLENGE_RAY_H
