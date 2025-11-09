use crate::{
    intersections::{intersection, Intersection},
    rays::Ray,
    shape::{Shape, ShapeType},
    tuple::{dot, point, Tuple},
};

#[derive(Clone, Debug)]
pub struct Sphere {}

impl Sphere {
    pub fn local_intersect(s: &Shape, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin.clone() - point(0.0, 0.0, 0.0);
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        vec![intersection(t1, s), intersection(t2, s)]
    }

    pub fn local_normal(_shape: &Shape, pt: &Tuple) -> Tuple {
        pt - point(0.0, 0.0, 0.0)
    }
}

pub fn sphere() -> Shape {
    Shape::new(ShapeType::Sphere)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        rays::ray,
        transformations::{rotation_z, scaling, translation},
        tuple::{normalise, point, vector},
    };

    use super::*;

    #[test]
    fn different_spheres_are_not_equal() {
        let s1 = sphere();
        let s2 = sphere();
        assert_ne!(s1, s2);
    }

    #[test]
    fn same_sphere_is_equal() {
        let s1 = sphere();
        assert_eq!(s1, s1);
    }

    #[test]
    fn intersect_set_the_object() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn normal_on_sphere_on_x_axis() {
        let s = sphere();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_on_y_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_on_z_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_on_nonaxial_point() {
        let s = sphere();
        let v = 3.0_f64.sqrt() / 3.0;
        let n = s.normal_at(&point(v, v, v));
        assert_eq!(n, vector(v, v, v));
    }

    #[test]
    fn normal_is_a_normalised_vector() {
        let s = sphere();
        let v = 3.0_f64.sqrt() / 3.0;
        let n = s.normal_at(&point(v, v, v));
        assert_eq!(n, normalise(&n));
    }

    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(scaling(2.0, 2.0, 2.0));
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(translation(5.0, 0.0, 0.0));
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_translated_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        s.set_transform(translation(0.0, 1.0, 0.0));

        let n = Shape::normal_at(&s, &point(0.0, 1.70711, -0.70711));

        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let mut s = Shape::new(ShapeType::Sphere);
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m);

        let n = Shape::normal_at(&s, &point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}
