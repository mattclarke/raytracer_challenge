#ifndef RAYTRACER_CHALLENGE_CANVAS_H
#define RAYTRACER_CHALLENGE_CANVAS_H

#include <algorithm>
#include <string>
#include <tuple>
#include <vector>

#include <fmt/core.h>

#include "tuple.h"

struct Canvas {
    Canvas(size_t width, size_t height):
        width(width), height(height), 
        pixels(width * height, Color{0, 0, 0}) {
    }

    size_t width;
    size_t height;
    std::vector<Color> pixels;
};

void write_pixel(Canvas &c, size_t x, size_t y, Color col) {
    c.pixels[c.width * y + x] = col;
}

Color pixel_at(const Canvas &c, size_t x, size_t y) {
    return c.pixels[c.width * y + x];
}

int scale_pixel_color(double color) {
    return std::clamp(static_cast<int>(std::ceil(color * 255)), 0, 255);
}

void add_rgb_element(std::string &overall, std::string &current, const std::string& element) {
    if (current.size() == 69) {
        overall += current + '\n';
        current = "";
    }
    if (current.size() + element.size() >= 69) {
        overall += current + '\n';
        current = element;
    } else {
        current += ' ' + element;
    }
}

std::string canvas_to_ppm(const Canvas &canvas) {
    std::string result = fmt::format("P3\n{} {}\n255\n", canvas.width, canvas.height);
    for (size_t y = 0; y < canvas.height; ++y) {
        std::string current;
        for (size_t x = 0; x < canvas.width; ++x) {
            auto pixel = canvas.pixels[canvas.width * y + x];
            auto red = std::to_string(scale_pixel_color(pixel.red));
            auto green = std::to_string(scale_pixel_color(pixel.green));
            auto blue = std::to_string(scale_pixel_color(pixel.blue));

            if (x == 0) {
                current += fmt::format("{} {} {}", red, green, blue);
                continue;
            }
            add_rgb_element(result, current, red);
            add_rgb_element(result, current, green);
            add_rgb_element(result, current, blue);
        }
        current += '\n';
        result += current;
    }
    return result;
}

#endif //RAYTRACER_CHALLENGE_CANVAS_H
