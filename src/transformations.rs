use crate::matrix::{inverse, Matrix};
use crate::tuple::{cross, normalise, point, vector, Tuple};
use std::f32::consts::PI;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn rotation_x(r: f32) -> Matrix {
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

pub fn rotation_y(r: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            r.cos(),
            0.0,
            r.sin(),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            -r.sin(),
            0.0,
            r.cos(),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn rotation_z(r: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
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
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ],
    )
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    )
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = normalise(&(to - from));
    let upn = normalise(&up);
    let left = cross(&forward, &upn);
    let true_up = cross(&left, &forward);
    let orientation = Matrix::new(
        4,
        4,
        vec![
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
            -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
    orientation * translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_by_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiple_by_inverse_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(inverse(&transform) * p, point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * &v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiple_by_inverse_of_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = inverse(&transform);
        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter * &p,
            point(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotating_point_around_y() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            half_quarter * &p,
            point(f32::sqrt(2.0) / 2.0, 0.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            half_quarter * &p,
            point(-f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(t * p, point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(t * p, point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(t * p, point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(t * p, point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(t * p, point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let p = point(2.0, 3.0, 4.0);
        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(t * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, point(15.0, 0.0, 7.0));
    }

    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, Matrix::identity_4x4());
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(&from, &to, &up);
        let expected = Matrix::new(
            4,
            4,
            vec![
                -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        assert_eq!(t, expected);
    }
}
