#ifndef RAYTRACER_CHALLENGE_WORLD_H
#define RAYTRACER_CHALLENGE_WORLD_H

#include <memory>
#include <vector>

#include "intersection.h"
#include "light.h"
#include "ray.h"
#include "sphere.h"
#include "transformations.h"

struct World {
    std::vector<Sphere> objects;
    std::unique_ptr<PointLight> lightsource{nullptr};
};


std::unique_ptr<World> create_default_world() {
    auto light = std::make_unique<PointLight>(point(0, 0, 0), Color{1, 1, 1});
    auto sphere1 = Sphere{1};
    sphere1.material = Material{};
    sphere1.material.color = Color{0.8, 1.0, 0.6};
    sphere1.material.diffuse = 0.7f;
    sphere1.material.specular = 0.2f;
    auto sphere2 = Sphere{2};
    set_transform(sphere2, scaling(0.5, 0.5, 0.5));

    auto w = std::make_unique<World>();
    w->lightsource = std::move(light);
    w->objects.push_back(sphere1);
    w->objects.push_back(sphere2);

    return w;
}

std::vector<Intersection> intersect_world(World const &world, Ray const &ray) {
    std::vector<Intersection> intersections;
    for (auto const &object : world.objects) {
        auto local = intersect(object, ray);
        for (auto const &i : local) {
            intersections.push_back(i);
        }
    }
    return intersections;
}

#endif //RAYTRACER_CHALLENGE_WORLD_H
