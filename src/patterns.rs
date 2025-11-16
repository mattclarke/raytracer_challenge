use crate::{color::Color, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b }
    }

    pub fn at(&self, point: &Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, tuple::point};

    use super::*;

    #[test]
    fn creating_stripe_pattern() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.a, Color::white());
        assert_eq!(pattern.b, Color::black());
    }

    #[test]
    fn stripe_is_constant_in_y() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.at(&point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.at(&point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn stripe_is_constant_in_z() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.at(&point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.at(&point(0.0, 2.0, 2.0)), Color::white());
    }

    #[test]
    fn stripe_alternates_in_x() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(pattern.at(&point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.at(&point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.at(&point(-1.1, 0.0, 0.0)), Color::white());
    }
}
