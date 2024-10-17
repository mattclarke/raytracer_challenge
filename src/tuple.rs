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
}
