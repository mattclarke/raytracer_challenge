#include <fstream>
#include <iostream>

#include "canvas.h"
#include "intersection.h"
#include "light.h"
#include "material.h"
#include "ray.h"
#include "sphere.h"
#include "tuple.h"
#include "transformations.h"

int main() {
    std::cout << "Generating..." << std::endl;

    size_t canvas_size = 100;
    auto wall_z = 10.0f;
    auto wall_size = 7.0f;
    auto pixel_size = wall_size / canvas_size;
    auto half = wall_size / 2.0f;
    auto canvas = Canvas{canvas_size, canvas_size};
    auto sphere = Sphere{1};
    sphere.material = Material{};
    sphere.material.color = Color{1, 0.2, 1};
    auto light_position = point(-10, 10, -10);
    auto light_color = Color{1, 1, 1};
    auto light = PointLight{light_position, light_color};
    // set_transform(sphere, scaling(0.5, 1, 1));
    auto ray_origin = point(0, 0, -5);

    for (size_t y = 0; y < canvas_size; ++y) {
        auto world_y = half - pixel_size * y;
        for (size_t x = 0; x < canvas_size; ++x) {
            auto world_x = -half + pixel_size * x;
            auto pos = point(world_x, world_y, wall_z);

            auto ray = Ray{ray_origin, normalise(pos - ray_origin)};
            auto xs = intersect(sphere, ray);
            auto hit_ = hit(xs);

            if (hit_) {
                auto point = position(ray, hit_->t);
                auto normal = normal_at(hit_->object, point);
                auto eye = -ray.direction;
                auto color = lighting(hit_->object.material, light, point, eye, normal);
                write_pixel(canvas, x, y, color);
            }
        }
    }

    auto contents = canvas_to_ppm(canvas);

    std::ofstream os;
    os.open("example.ppm");
    os << contents << std::endl;
    os.close();

    std::cout << "Done!" << std::endl;

    return 0;
}
