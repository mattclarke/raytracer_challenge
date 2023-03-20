#ifndef RAYTRACER_CHALLENGE_INTERSECTION_H
#define RAYTRACER_CHALLENGE_INTERSECTION_H

#include <cmath>
#include <optional>
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

bool operator==(const Intersection &lhs, const Intersection &rhs) {
    return (lhs.t == rhs.t) && (lhs.object == rhs.object);
}

std::vector<Intersection> intersections(std::initializer_list<Intersection> items) {
    auto result = std::vector<Intersection>(items);

    std::sort(result.begin(), result.end(), [](const auto &a, const auto &b)
              {
                  return a.t < b.t;
              });

    return result;
}

std::optional<Intersection> hit(const std::vector<Intersection> &intersections){
    for(const auto & i : intersections) {
        if (i.t > 0) {
            return i;
        }
    }
    return std::nullopt;
}

#endif // RAYTRACER_CHALLENGE_INTERSECTION_H
