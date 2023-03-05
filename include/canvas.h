#ifndef RAYTRACER_CHALLENGE_CANVAS_H
#define RAYTRACER_CHALLENGE_CANVAS_H

#include <algorithm>
#include <string>
#include <vector>

#include <fmt/core.h>

#include "tuple.h"

struct Canvas {
    Canvas(size_t width, size_t height):
        width(width), height(height), 
        pixels(width * height, Colour{0, 0, 0}) {
    }

    size_t width;
    size_t height;
    std::vector<Colour> pixels;
};

void write_pixel(Canvas &c, size_t x, size_t y, Colour col) {
    c.pixels[c.width * y + x] = col;
}

Colour pixel_at(const Canvas &c, size_t x, size_t y) {
    return c.pixels[c.width * y + x];
}

int scale_pixel_colour(double colour) {
    return std::clamp(static_cast<int>(std::ceil(colour * 255)), 0, 255);
}

std::string canvas_to_ppm(const Canvas &canvas) {
    std::string result = fmt::format("P3\n{} {}\n255\n", canvas.width, canvas.height);
    size_t i = 0;
    for (const auto &p : canvas.pixels) {
        result += fmt::format("{} {} {}", scale_pixel_colour(p.red), scale_pixel_colour(p.green), scale_pixel_colour(p.blue));
        ++i;
        if (i % canvas.width == 0) {
            result += '\n';
        }
        else {
            result += ' ';
        }
    }
    std::cout << result;
    return result;
}

#endif //RAYTRACER_CHALLENGE_CANVAS_H
