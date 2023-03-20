#include <fstream>
#include <iostream>

#include "canvas.h"
#include "ray.h"
#include "tuple.h"

int main() {
    std::cout << "Generating..." << std::endl;

    size_t canvas_size = 100;
    auto wall_z = 10.0f;
    auto wall_size = 7.0f;
    auto pixel_size = wall_size / canvas_size;
    auto half = wall_size / 2.0f;
    auto canvas = Canvas{canvas_size, canvas_size};
    auto clr = Colour{1.0, 0.0, 0.0};
    auto shape = Sphere{1};
    auto ray_origin = point(0, 0, -5);

    for (size_t y = 0; y < canvas_size; ++y) {
        auto world_y = half - pixel_size * y;
        for (size_t x = 0; x < canvas_size; ++x) {
            auto world_x = -half + pixel_size * x;
            auto position = point(world_x, world_y, wall_z);

            auto ray = Ray{ray_origin, normalise(position - ray_origin)};
            auto xs = intersect(shape, ray);
            
            if (hit(xs)) {
                write_pixel(canvas, x, y, clr);
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
