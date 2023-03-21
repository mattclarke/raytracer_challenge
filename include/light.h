#ifndef RAYTRACER_CHALLENGE_LIGHT_H
#define RAYTRACER_CHALLENGE_LIGHT_H

#include <cassert>
#include <vector>

#include "material.h"
#include "tuple.h"
#include "utils.h"

struct PointLight {
    Tuple position;
    Colour intensity;

    PointLight(Tuple position, Colour intensity) :
        position(position), intensity(intensity) {
    }
};

Colour lighting(const Material &material, const PointLight &light, const Tuple &point, const Tuple &eyev, const Tuple &normalv) {
    auto effective_colour = material.colour * light.intensity;auto lightv = normalise(light.position - point);
    auto ambient = effective_colour * material.ambient;
    auto light_dot_normal = dot(lightv, normalv);
    auto diffuse = Colour{0, 0, 0};
    auto specular = Colour{0, 0, 0};

    if (light_dot_normal >= 0) {
        diffuse = effective_colour * material.diffuse * light_dot_normal;

        auto reflectv = reflect(-lightv, normalv);
        auto reflect_dot_eye = dot(reflectv, eyev);

        if (reflect_dot_eye > 0) {
            auto factor = std::pow(reflect_dot_eye, material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }
    return ambient + diffuse + specular;
}

#endif //RAYTRACER_CHALLENGE_LIGHT_H
