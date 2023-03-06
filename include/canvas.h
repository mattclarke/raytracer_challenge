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
    size_t line_length = 0;
    for (size_t y = 0; y < canvas.height; ++y) {
        for (size_t x = 0; x < canvas.width; ++x) {
            auto pixel = canvas.pixels[canvas.width * y + x];
            auto s_red = std::to_string(scale_pixel_colour(pixel.red));
            auto s_green = std::to_string(scale_pixel_colour(pixel.green));
            auto s_blue = std::to_string(scale_pixel_colour(pixel.blue));

            if (x == 0) {
                result += s_red + ' ' + s_green + ' ' + s_blue;
            } else {
                result += ' ' + s_red + ' ' + s_green + ' ' + s_blue;
            }

        }
        result += '\n';
        line_length = 0;
    }
    return result;
}

#endif //RAYTRACER_CHALLENGE_CANVAS_H
