#ifndef RAYTRACER_CHALLENGE_WORLD_H
#define RAYTRACER_CHALLENGE_WORLD_H

#include <vector>

#include "light.h"
#include "sphere.h"

struct World {
    std::vector<Sphere> objects;
    PointLight* lightsource{nullptr};
};

#endif //RAYTRACER_CHALLENGE_WORLD_H
