use crate::matrix::{inverse, Matrix};
use crate::tuple::{point, vector};
use std::f32::consts::PI;

fn translation(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

fn rotation_x(r: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            r.cos(),
            -r.sin(),
            0.0,
            0.0,
            r.sin(),
            r.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &p, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiple_by_inverse_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(&inverse(&transform) * &p, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(&transform * &v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &p, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(&transform * &v, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiple_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = inverse(&transform);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(&inv * &v, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(&transform * &p, point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            point(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(&full_quarter * &p, point(0.0, 0.0, 1.0));
    }
}
