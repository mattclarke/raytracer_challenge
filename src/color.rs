use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub red: f32,
    pub blue: f32,
    pub green: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c1 + c2;
        assert_relative_eq!(result.red, 1.6);
        assert_relative_eq!(result.green, 0.7);
        assert_relative_eq!(result.blue, 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c1 - c2;
        assert_relative_eq!(result.red, 0.2);
        assert_relative_eq!(result.green, 0.5);
        assert_relative_eq!(result.blue, 0.5);
    }

    #[test]
    fn multiplying_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let result = c * 2.0;
        assert_relative_eq!(result.red, 0.4);
        assert_relative_eq!(result.green, 0.6);
        assert_relative_eq!(result.blue, 0.8);
    }

    #[test]
    fn multiplying_color_by_a_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let result = c1 * c2;
        assert_relative_eq!(result.red, 0.9);
        assert_relative_eq!(result.green, 0.2);
        assert_relative_eq!(result.blue, 0.04);
    }
}
