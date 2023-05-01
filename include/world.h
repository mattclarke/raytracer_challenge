#ifndef RAYTRACER_CHALLENGE_WORLD_H
#define RAYTRACER_CHALLENGE_WORLD_H

#include <vector>

struct World {
    std::vector<int> objects;
    int* lightsource{nullptr};
};

#endif //RAYTRACER_CHALLENGE_WORLD_H
