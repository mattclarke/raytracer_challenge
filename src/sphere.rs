use crate::{
    intersections::{intersection, Intersection},
    matrix::inverse,
    rays::{transform, Ray},
    shape::{Shape, ShapeType},
    tuple::{dot, normalise, point, Tuple},
};

#[derive(Clone, Debug)]
pub struct Sphere {}

impl Sphere {
    pub fn normal_at(s: &Shape, p: &Tuple) -> Tuple {
        let obj_point = inverse(s.transform()) * p;
        let obj_normal = obj_point - point(0.0, 0.0, 0.0);
        let mut world_normal = inverse(s.transform()).transpose() * obj_normal;
        world_normal.w = 0.0;
        normalise(&world_normal)
    }

    pub fn intersect(s: &Shape, ray: &Ray) -> Vec<Intersection> {
        let r = transform(ray, &inverse(s.transform()));

        let sphere_to_ray = r.origin.clone() - point(0.0, 0.0, 0.0);
        let a = dot(&r.direction, &r.direction);
        let b = 2.0 * dot(&r.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        vec![intersection(t1, s), intersection(t2, s)]
    }
}

pub fn sphere() -> Shape {
    Shape::new(ShapeType::Sphere)
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::ray,
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
}
