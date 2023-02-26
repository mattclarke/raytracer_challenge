#ifndef RAYTRACER_CHALLENGE_CANVAS_H
#define RAYTRACER_CHALLENGE_CANVAS_H

#include <vector>

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

#endif //RAYTRACER_CHALLENGE_CANVAS_H
