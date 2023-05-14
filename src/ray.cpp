#include <cmath>

#include "ray.h"

#include "intersection.h"
#include "matrix.h"
#include "sphere.h"
#include "tuple.h"
#include "utils.h"

Tuple position(const Ray &ray, float t) {
    return ray.origin + ray.direction * t;
}

Ray transform(const Ray &r, const Matrix &m) {
    return {m * r.origin, m * r.direction};
}

std::vector<Intersection> intersect(const Sphere &s, const Ray &r) {
    auto ray = transform(r, inverse(s.transform));
    auto sphere_to_ray = ray.origin - point(0, 0, 0);

    auto a = dot(ray.direction, ray.direction);
    auto b = 2 * dot(ray.direction, sphere_to_ray);
    auto c = dot(sphere_to_ray, sphere_to_ray) - 1;

    auto discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return {};
    }

    auto t1 = static_cast<float>((-b - std::sqrt(discriminant)) / (2 * a));
    auto t2 = static_cast<float>((-b + std::sqrt(discriminant)) / (2 * a));

    return {Intersection{t1, s}, Intersection{t2, s}};
}

