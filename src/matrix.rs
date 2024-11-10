use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
struct Matrix {
    width: usize,
    height: usize,
    elements: Vec<f32>,
}

impl Matrix {
    pub fn new(width: usize, height: usize, values: Vec<f32>) -> Matrix {
        assert!(
            values.len() == width * height,
            "number of values does not match dimensions"
        );
        Matrix {
            width,
            height,
            elements: values,
        }
    }

    pub fn at(&self, y: usize, x: usize) -> f32 {
        self.elements[y * self.width + x]
    }

    pub fn identity_4x4() -> Matrix {
        Matrix::new(
            4,
            4,
            vec![
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height, vec![0.0; self.height * self.width]);
        for row in 0..self.height {
            for col in 0..self.width {
                result.elements[row * self.width + col] = self.at(col, row);
            }
        }
        result
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        assert!(
            self.width == rhs.width && self.height == rhs.height,
            "Cannot multiply different shaped matrices"
        );
        let mut result = Matrix::new(self.width, self.height, vec![0.0; self.height * self.width]);

        for row in 0..self.height {
            for col in 0..self.width {
                result.elements[row * self.width + col] = self.at(row, 0) * rhs.at(0, col)
                    + self.at(row, 1) * rhs.at(1, col)
                    + self.at(row, 2) * rhs.at(2, col)
                    + self.at(row, 3) * rhs.at(3, col);
            }
        }

        result
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.at(0, 0) * rhs.x
                + self.at(0, 1) * rhs.y
                + self.at(0, 2) * rhs.z
                + self.at(0, 3) * rhs.w,
            y: self.at(1, 0) * rhs.x
                + self.at(1, 1) * rhs.y
                + self.at(1, 2) * rhs.z
                + self.at(1, 3) * rhs.w,
            z: self.at(2, 0) * rhs.x
                + self.at(2, 1) * rhs.y
                + self.at(2, 2) * rhs.z
                + self.at(2, 3) * rhs.w,
            w: self.at(3, 0) * rhs.x
                + self.at(3, 1) * rhs.y
                + self.at(3, 2) * rhs.z
                + self.at(3, 3) * rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_4x4_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );
        assert_eq!(matrix.at(0, 0), 1.0);
        assert_eq!(matrix.at(0, 3), 4.0);
        assert_eq!(matrix.at(1, 0), 5.5);
        assert_eq!(matrix.at(1, 2), 7.5);
        assert_eq!(matrix.at(2, 2), 11.0);
        assert_eq!(matrix.at(3, 0), 13.5);
        assert_eq!(matrix.at(3, 2), 15.5);
    }

    #[test]
    fn can_represent_2x2_matrix() {
        let matrix = Matrix::new(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);
        assert_eq!(matrix.at(0, 0), -3.0);
        assert_eq!(matrix.at(0, 1), 5.0);
        assert_eq!(matrix.at(1, 0), 1.0);
        assert_eq!(matrix.at(1, 1), -2.0);
    }

    #[test]
    fn can_represent_3x3_matrix() {
        let matrix = Matrix::new(3, 3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);
        assert_eq!(matrix.at(0, 0), -3.0);
        assert_eq!(matrix.at(1, 1), -2.0);
        assert_eq!(matrix.at(2, 2), 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );
        let m2 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );
        let m2 = Matrix::new(
            4,
            4,
            vec![
                2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
                1.0,
            ],
        );
        assert_ne!(m1, m2);
    }

    #[test]
    fn multiply_two_matrices() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m2 = Matrix::new(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );
        let expected = Matrix::new(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );
        assert_eq!(&m1 * &m2, expected);
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(
            &matrix * &tuple,
            Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn multiply_matrix_by_identity() {
        let identity = Matrix::identity_4x4();
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        assert_eq!(&matrix * &identity, matrix);
    }

    #[test]
    fn multiply_identity_by_tuple() {
        let identity = Matrix::identity_4x4();
        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        assert_eq!(&identity * &t, t);
    }

    #[test]
    fn transposing_a_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );
        let expected = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );
        assert_eq!(matrix.transpose(), expected);
    }

    #[test]
    fn transpose_identity() {
        let identity = Matrix::identity_4x4();
        assert_eq!(identity.transpose(), identity);
    }
}
