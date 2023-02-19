#include <cmath>

struct Tuple {
    double x;
    double y;
    double z;
    double w;

    bool is_point() {
        return w == 1.0;
    }

    bool is_vector() {
        return w == 0.0;
    }

};

bool operator==(const Tuple &lhs, const Tuple &rhs) {
    // TODO: use epsilon for comparison?
    return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z && lhs.w == rhs.w;
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
