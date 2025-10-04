use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w != 1.0
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Add<f32> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
            w: self.w + rhs,
        }
    }
}

impl Sub for &Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<Tuple> for &Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Sub<f32> for Tuple {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
            w: self.w - rhs,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<f32> for &Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Tuple> for f32 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
            w: self * rhs.w,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        if (self.x - other.x).abs() > epsilon {
            return false;
        }
        if (self.y - other.y).abs() > epsilon {
            return false;
        }
        if (self.z - other.z).abs() > epsilon {
            return false;
        }
        if (self.w - other.w).abs() > epsilon {
            return false;
        }
        return true;
    }
}

impl Eq for Tuple {}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn magnitude(tuple: &Tuple) -> f32 {
    let pows = tuple.x.powf(2.0) + tuple.y.powf(2.0) + tuple.z.powf(2.0);
    pows.sqrt()
}

pub fn normalise(tuple: &Tuple) -> Tuple {
    let magn = magnitude(&tuple);
    Tuple {
        x: tuple.x / magn,
        y: tuple.y / magn,
        z: tuple.z / magn,
        w: tuple.w / magn,
    }
}

pub fn dot(a: &Tuple, b: &Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    // Note: w is unused - this is only used for vectors
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

pub fn reflect(v: &Tuple, normal: &Tuple) -> Tuple {
    let temp = normal * 2.0 * dot(&v, &normal);
    v - &temp
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn tuple_with_w_equals_one_is_a_point() {
        let result = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };
        assert_eq!(result.x, 4.3);
        assert_eq!(result.y, -4.2);
        assert_eq!(result.z, 3.1);
        assert_eq!(result.w, 1.0);
        assert_eq!(result.is_point(), true);
        assert_eq!(result.is_vector(), false);
    }

    #[test]
    fn tuple_with_w_not_equals_one_is_a_vector() {
        let result = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };
        assert_eq!(result.x, 4.3);
        assert_eq!(result.y, -4.2);
        assert_eq!(result.z, 3.1);
        assert_eq!(result.w, 0.0);
        assert_eq!(result.is_point(), false);
        assert_eq!(result.is_vector(), true);
    }

    #[test]
    fn point_creates_tuple_with_w_equal_one() {
        let result = point(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn vector_creates_tuple_with_w_equal_zero() {
        let result = vector(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn adding_two_tuples() {
        let t1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let t2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        assert_eq!(
            t1 + t2,
            Tuple {
                x: 1.0,
                y: 1.0,
                z: 6.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_zero_vector() {
        let v = vector(1.0, -2.0, 3.0);
        let zero = vector(0.0, 0.0, 0.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            -a,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0
            }
        );
    }

    #[test]
    fn multiple_tuple_by_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            a * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
    }

    #[test]
    fn multiple_tuple_by_fraction() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            a * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };
        assert_eq!(
            a / 2.0,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_0_0() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_1_0() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_0_1() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(&v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(magnitude(&v), 14.0_f32.sqrt());
    }

    #[test]
    fn compute_the_magnitude_of_vector_negative_1_2_3() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(magnitude(&v), 14.0_f32.sqrt());
    }

    #[test]
    fn normalise_4_0_0() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(normalise(&v), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalise_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        let result = normalise(&v);
        assert_relative_eq!(result.x, 0.26726, epsilon = 0.00001);
        assert_relative_eq!(result.y, 0.53452, epsilon = 0.00001);
        assert_relative_eq!(result.z, 0.80178, epsilon = 0.00001);
        assert_relative_eq!(result.w, 0.0, epsilon = 0.00001);
    }

    #[test]
    fn magnitude_of_normalised() {
        let v = vector(1.0, 2.0, 3.0);
        assert_relative_eq!(magnitude(&normalise(&v)), 1.0, epsilon = 0.00001);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_relative_eq!(dot(&v1, &v2), 20.0, epsilon = 0.00001);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(cross(&v1, &v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(&v2, &v1), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_vector_approaching_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector(2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
