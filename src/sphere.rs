#[derive(Debug)]
pub struct Sphere {}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

pub fn sphere() -> Sphere {
    Sphere {}
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::{intersect, ray},
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn different_spheres_are_not_equal() {
        let s1 = Sphere {};
        let s2 = Sphere {};
        assert_ne!(s1, s2);
    }

    #[test]
    fn same_sphere_is_equal() {
        let s1 = Sphere {};
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
}
