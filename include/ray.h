#ifndef RAYTRACER_CHALLENGE_RAY_H
#define RAYTRACER_CHALLENGE_RAY_H

#include <cmath>
#include <vector>

#include "intersection.h"
#include "matrix.h"
#include "sphere.h"
#include "tuple.h"
#include "utils.h"

struct Ray {
    Tuple origin;
    Tuple direction;

    Ray(const Tuple &origin, const Tuple &direction) :
        origin(origin), direction(direction) 
    {}
};

Tuple position(const Ray &ray, float t) {
    return ray.origin + ray.direction * t;
}

std::vector<Intersection> intersect(const Sphere &s, const Ray &r) {
    auto sphere_to_ray = r.origin - point(0, 0, 0);

    auto a = dot(r.direction, r.direction);
    auto b = 2 * dot(r.direction, sphere_to_ray);
    auto c = dot(sphere_to_ray, sphere_to_ray) - 1;

    auto discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return {};
    }

    auto t1 = static_cast<float>((-b - std::sqrt(discriminant)) / (2 * a));
    auto t2 = static_cast<float>((-b + std::sqrt(discriminant)) / (2 * a));

    return {Intersection{t1, s}, Intersection{t2, s}};
}

Ray transform(const Ray &r, const Matrix &m) {
    return {m * r.origin, m * r.direction};
}
#endif // RAYTRACER_CHALLENGE_RAY_H
