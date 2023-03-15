#ifndef RAYTRACER_CHALLENGE_UTILS_H
#define RAYTRACER_CHALLENGE_UTILS_H

#include <cmath>

bool equal(const double a, const double b){
    const static double epsilon = 0.00001; 
    if (std::abs (a-b) < epsilon) return true;
    return false;
}

#endif // RAYTRACER_CHALLENGE_UTILS_H
