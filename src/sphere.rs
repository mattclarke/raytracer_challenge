use crate::{
    matrix::Matrix,
    tuple::{normalise, point, Tuple},
};

#[derive(Debug)]
pub struct Sphere {
    pub transform: Matrix,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

pub fn sphere() -> Sphere {
    Sphere {
        transform: Matrix::identity_4x4(),
    }
}

fn normal_at(s: &Sphere, p: &Tuple) -> Tuple {
    normalise(&(p - &point(0.0, 0.0, 0.0)))
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::Matrix,
        rays::{intersect, ray},
        transformations::{scaling, translation},
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
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = sphere();
        assert_eq!(s.transform, Matrix::identity_4x4());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = sphere();
        let t = translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.transform = scaling(2.0, 2.0, 2.0);
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.transform = translation(5.0, 0.0, 0.0);
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_on_sphere_on_x_axis() {
        let s = sphere();
        let n = normal_at(&s, &point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_on_y_axis() {
        let s = sphere();
        let n = normal_at(&s, &point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_on_z_axis() {
        let s = sphere();
        let n = normal_at(&s, &point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_on_nonaxial_point() {
        let s = sphere();
        let v = 3.0_f32.sqrt() / 3.0;
        let n = normal_at(&s, &point(v, v, v));
        assert_eq!(n, vector(v, v, v));
    }

    #[test]
    fn normal_is_a_normalised_vector() {
        let s = sphere();
        let v = 3.0_f32.sqrt() / 3.0;
        let n = normal_at(&s, &point(v, v, v));
        assert_eq!(n, normalise(&n));
    }
}
