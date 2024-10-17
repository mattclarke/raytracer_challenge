use std::ops::{Add, Neg, Sub};

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
            w: self.w + rhs.w
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
            w: self.w - rhs.w
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
            w: -self.w
        }
    }
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 1.0,
    }
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: 0.0,
    }
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
        assert_eq!(t1 + t2, Tuple{x: 1.0, y: 1.0, z: 6.0, w: 1.0});
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
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        assert_eq!(-a, Tuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0});
    }
}
