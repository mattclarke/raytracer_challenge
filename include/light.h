#ifndef RAYTRACER_CHALLENGE_LIGHT_H
#define RAYTRACER_CHALLENGE_LIGHT_H

#include <cassert>
#include <vector>

#include "tuple.h"
#include "utils.h"

struct PointLight {
    Tuple position;
    Colour intensity;

    PointLight(Tuple position, Colour intensity) :
        position(position), intensity(intensity) {
    }
};

#endif //RAYTRACER_CHALLENGE_LIGHT_H
