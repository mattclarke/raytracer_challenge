#ifndef RAYTRACER_CHALLENGE_MATERIAL_H
#define RAYTRACER_CHALLENGE_MATERIAL_H

#include <cassert>
#include <vector>

#include "tuple.h"
#include "utils.h"

struct Material {
    float ambient = 0.1f;
    float diffuse = 0.9f;
    float specular = 0.9f;
    float shininess = 200.0f;
    Colour colour = Colour{1, 1, 1};
};

bool operator==(const Material &lhs, const Material &rhs) {
    return (lhs.ambient == rhs.ambient) && (lhs.diffuse == rhs.diffuse) && (lhs.specular == rhs.specular) && (lhs.shininess == rhs.shininess) && (lhs.colour == rhs.colour);
}

#endif //RAYTRACER_CHALLENGE_MATERIAL_H
