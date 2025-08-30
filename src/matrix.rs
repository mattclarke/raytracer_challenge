use crate::tuple::Tuple;
use std::iter::zip;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    elements: Vec<f32>,
}

fn determinant(matrix: &Matrix) -> f32 {
    let mut result = 0.0;
    if matrix.width == 2 && matrix.height == 2 {
        result = matrix.elements[0] * matrix.elements[3] - matrix.elements[1] * matrix.elements[2];
    } else {
        for c in 0..matrix.width {
            result += matrix.at(0, c) * cofactor(&matrix, 0, c);
        }
    }
    result
}

fn invertible(matrix: &Matrix) -> bool {
    determinant(&matrix) != 0.0
}

pub fn inverse(matrix: &Matrix) -> Matrix {
    assert!(invertible(&matrix), "Not invertible");
    let det = determinant(&matrix);
    let mut result = Matrix::new(
        matrix.width,
        matrix.height,
        vec![0.0; matrix.width * matrix.height],
    );
    for r in 0..matrix.height {
        for c in 0..matrix.width {
            result.elements[c * result.width + r] = cofactor(&matrix, r, c) / det;
        }
    }
    result
}

fn submatrix(matrix: &Matrix, row: usize, column: usize) -> Matrix {
    let mut result = Matrix::new(
        matrix.width - 1,
        matrix.height - 1,
        vec![0.0; (matrix.width - 1) * (matrix.height - 1)],
    );
    let mut index: usize = 0;
    for y in 0..matrix.height {
        if y == row {
            continue;
        }
        for x in 0..matrix.width {
            if x == column {
                continue;
            }
            result.elements[index] = matrix.at(y, x);
            index += 1;
        }
    }
    result
}

fn minor(matrix: &Matrix, row: usize, column: usize) -> f32 {
    determinant(&submatrix(&matrix, row, column))
}

fn cofactor(matrix: &Matrix, row: usize, column: usize) -> f32 {
    let mnr = minor(&matrix, row, column);
    if (row + column) % 2 == 1 {
        return -mnr;
    }
    mnr
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width || self.height != other.height {
            return false;
        }

        let epsilon = 0.00001;
        for (a, b) in zip(self.elements.iter(), other.elements.iter()) {
            if (a - b).abs() > epsilon {
                return false;
            }
        }
        return true;
    }
}

impl Eq for Matrix {}

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

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
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

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self * &rhs
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
        assert_eq!(m1 * m2, expected);
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
            matrix * tuple,
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
        assert_eq!(&matrix * identity, matrix);
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
        assert_eq!(identity * &t, t);
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

    #[test]
    fn calculate_determinant_of_2x2_matrix() {
        let matrix = Matrix::new(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        assert_eq!(determinant(&matrix), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let matrix = Matrix::new(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let expected = Matrix::new(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);
        assert_eq!(submatrix(&matrix, 0, 2), expected);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );
        let expected = Matrix::new(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);
        assert_eq!(submatrix(&matrix, 2, 1), expected);
    }

    #[test]
    fn calculate_minor_of_3x3() {
        let a = Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let b = submatrix(&a, 1, 0);
        assert_eq!(determinant(&b), 25.0);
        assert_eq!(minor(&a, 1, 0), 25.0);
    }

    #[test]
    fn calculate_cofactor_of_3x3() {
        let a = Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        assert_eq!(cofactor(&a, 0, 0), -12.0);
        assert_eq!(cofactor(&a, 1, 0), -25.0);
    }

    #[test]
    fn calculate_determinant_of_3x3() {
        let a = Matrix::new(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
        assert_eq!(determinant(&a), -196.0);
    }

    #[test]
    fn calculate_determinant_of_4x4() {
        let a = Matrix::new(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );
        assert_eq!(determinant(&a), -4071.0);
    }

    #[test]
    fn is_invertible() {
        let a = Matrix::new(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );
        assert_eq!(determinant(&a), -2120.0);
        assert_eq!(invertible(&a), true);
    }

    #[test]
    fn is_not_invertible() {
        let a = Matrix::new(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );
        assert_eq!(determinant(&a), 0.0);
        assert_eq!(invertible(&a), false);
    }

    #[test]
    fn calculate_matrix_inverse_1() {
        let a = Matrix::new(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );
        let b = inverse(&a);
        let expected = Matrix::new(
            4,
            4,
            vec![
                0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
            ],
        );
        assert_eq!(determinant(&a), 532.0);
        assert_eq!(cofactor(&a, 2, 3), -160.0);
        assert_eq!(cofactor(&a, 3, 2), 105.0);
        assert_eq!(b.at(3, 2), -160.0 / 532.0);
        assert_eq!(b.at(2, 3), 105.0 / 532.0);
        assert_eq!(b, expected);
    }

    #[test]
    fn calculate_matrix_inverse_2() {
        let a = Matrix::new(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );
        let b = inverse(&a);
        let expected = Matrix::new(
            4,
            4,
            vec![
                -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
            ],
        );
        assert_eq!(b, expected);
    }

    #[test]
    fn calculate_matrix_inverse_3() {
        let a = Matrix::new(
            4,
            4,
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
        );
        let b = inverse(&a);
        let expected = Matrix::new(
            4,
            4,
            vec![
                -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
            ],
        );
        assert_eq!(b, expected);
    }

    #[test]
    fn multiply_product_by_inverse() {
        let a = Matrix::new(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );
        let b = Matrix::new(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );
        let c = &a * &b;
        assert_eq!(c * inverse(&b), a);
    }
}
