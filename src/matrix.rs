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
}
