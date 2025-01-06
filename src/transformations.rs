use crate::matrix::{inverse, Matrix};
use crate::tuple::point;

fn translation(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(
        4,
        4,
        vec![
            1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
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
}
