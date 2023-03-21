#ifndef RAYTRACER_CHALLENGE_TUPLE_H
#define RAYTRACER_CHALLENGE_TUPLE_H

#include <cmath>

#include "utils.h"

struct Tuple {
    double x;
    double y;
    double z;
    double w;

    [[nodiscard]] bool is_point() const {
        return w == 1.0;
    }

    [[nodiscard]] bool is_vector() const {
        return w == 0.0;
    }

};

bool operator==(const Tuple &lhs, const Tuple &rhs) {
    return equal(lhs.x, rhs.x) && equal(lhs.y, rhs.y) && equal(lhs.z, rhs.z) && equal(lhs.w, rhs.w);
}

Tuple operator+(const Tuple &lhs, const Tuple &rhs) {
    return {lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w};
}

Tuple operator-(const Tuple &lhs, const Tuple &rhs) {
    return {lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w};
}

Tuple operator-(const Tuple &rhs) {
    return {-rhs.x, -rhs.y, -rhs.z, -rhs.w};
}

Tuple operator*(const Tuple &tuple, double scalar) {
    return {tuple.x * scalar, tuple.y * scalar, tuple.z * scalar, tuple.w * scalar};
}

Tuple operator/(const Tuple &tuple, double scalar) {
    return {tuple.x / scalar, tuple.y / scalar, tuple.z / scalar, tuple.w / scalar};
}

// Factory functions
Tuple point(double x, double y, double z) {
    return {x, y, z, 1.0};
}

Tuple vector(double x, double y, double z) {
    return {x, y, z, 0.0};
}

// Helper functions
double magnitude(const Tuple &tuple) {
    return std::sqrt(tuple.x * tuple.x + tuple.y * tuple.y + tuple.z * tuple.z + tuple.w * tuple.w);
}

Tuple normalise(const Tuple &tuple) {
    auto magn = magnitude(tuple); 
    return {tuple.x / magn, tuple.y / magn, tuple.z / magn, tuple.w / magn};
}

double dot(const Tuple &a, const Tuple &b) {
    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
}

Tuple cross(const Tuple &a, const Tuple &b) {
    return vector(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x);
}

struct Colour {
    double red;
    double green;
    double blue;
};

bool operator==(const Colour &lhs, const Colour &rhs) {
    return equal(lhs.red, rhs.red) && equal(lhs.green, rhs.green) && equal(lhs.blue, rhs.blue);
}

Colour operator+(const Colour &lhs, const Colour &rhs) {
    return {lhs.red + rhs.red, lhs.green + rhs.green, lhs.blue + rhs.blue};
}

Colour operator-(const Colour &lhs, const Colour &rhs) {
    return {lhs.red - rhs.red, lhs.green - rhs.green, lhs.blue - rhs.blue};
}

Colour operator*(const Colour &lhs, double scalar) {
    return {lhs.red * scalar, lhs.green * scalar, lhs.blue * scalar};
}

Colour operator*(const Colour &lhs, const Colour &rhs) {
    return {lhs.red * rhs.red, lhs.green * rhs.green, lhs.blue * rhs.blue};
}

Tuple reflect(const Tuple &in, const Tuple &normal){
    return in - normal * 2 * dot(in, normal);
}

#endif // RAYTRACER_CHALLENGE_TUPLE_H
