use crate::{
    color::Color,
    matrix::{inverse, Matrix},
    shape::Shape,
    tuple::Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Stripe {
    a: Color,
    b: Color,
    pub transformation: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe {
            a,
            b,
            transformation: Matrix::identity_4x4(),
        }
    }

    fn color_at_point(&self, point: &Tuple) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn at(&self, object: &Shape, point: &Tuple) -> Color {
        let object_point = inverse(object.transform()) * point;
        let pattern_point = inverse(&self.transformation) * object_point;
        self.color_at_point(&pattern_point)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        sphere::sphere,
        transformations::{scaling, translation},
        tuple::point,
    };

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
        assert_eq!(
            pattern.color_at_point(&point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(0.0, 1.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(0.0, 2.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_is_constant_in_z() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(
            pattern.color_at_point(&point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(0.0, 0.0, 1.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(0.0, 2.0, 2.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_alternates_in_x() {
        let pattern = Stripe::new(Color::white(), Color::black());
        assert_eq!(
            pattern.color_at_point(&point(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(0.9, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.color_at_point(&point(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.color_at_point(&point(-0.1, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.color_at_point(&point(-1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.color_at_point(&point(-1.1, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripe_with_object_transformation() {
        let mut object = sphere();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = Stripe::new(Color::white(), Color::black());
        let c = pattern.at(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripe_with_pattern_transformation() {
        let object = sphere();
        let mut pattern = Stripe::new(Color::white(), Color::black());
        pattern.transformation = scaling(2.0, 2.0, 2.0);
        let c = pattern.at(&object, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripe_with_pattern_and_object_transformation() {
        let mut object = sphere();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = Stripe::new(Color::white(), Color::black());
        pattern.transformation = translation(0.5, 0.0, 0.0);
        let c = pattern.at(&object, &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }
}
