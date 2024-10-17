use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
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

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
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

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

fn magnitude(tuple: Tuple) -> f32 {
    let pows = tuple.x.powf(2.0) + tuple.y.powf(2.0) + tuple.z.powf(2.0);
    pows.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_1_0() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_0_0_1() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(magnitude(v), 1.0);
    }

    #[test]
    fn compute_the_magnitude_of_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(magnitude(v), 14.0_f32.sqrt());
    }

    #[test]
    fn compute_the_magnitude_of_vector_negative_1_2_3() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(magnitude(v), 14.0_f32.sqrt());
    }
}
