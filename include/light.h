#ifndef RAYTRACER_CHALLENGE_LIGHT_H
#define RAYTRACER_CHALLENGE_LIGHT_H

#include <cassert>
#include <vector>

#include "material.h"
#include "tuple.h"
#include "utils.h"

struct PointLight {
    Tuple position;
    Color intensity;

    PointLight(Tuple position, Color intensity) :
        position(position), intensity(intensity) {
    }
};

Color lighting(const Material &material, const PointLight &light, const Tuple &point, const Tuple &eyev, const Tuple &normalv) {
    auto effective_color = material.color * light.intensity;auto lightv = normalise(light.position - point);
    auto ambient = effective_color * material.ambient;
    auto light_dot_normal = dot(lightv, normalv);
    auto diffuse = Color{0, 0, 0};
    auto specular = Color{0, 0, 0};

    if (light_dot_normal >= 0) {
        diffuse = effective_color * material.diffuse * light_dot_normal;

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
