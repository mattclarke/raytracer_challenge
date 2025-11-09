use crate::{
    intersections::{intersection, Intersection},
    matrix::{inverse, EPSILON},
    rays::{transform, Ray},
    shape::{Shape, ShapeType},
    tuple::{vector, Tuple},
};

pub struct Plane {}

impl Plane {
    pub fn local_intersect(s: &Shape, ray: &Ray) -> Vec<Intersection> {
        let ray = transform(ray, &inverse(s.transform()));

        if ray.direction.y.abs() < EPSILON {
            return Vec::new();
        }

        let t = -ray.origin.y / ray.direction.y;
        vec![intersection(t, s)]
    }

    pub fn local_normal(_shape: &Shape, _pt: &Tuple) -> Tuple {
        vector(0.0, 1.0, 0.0)
    }
}

pub fn plane() -> Shape {
    Shape::new(ShapeType::Plane)
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::ray,
        shape::{Shape, ShapeType},
        tuple::{point, vector},
    };

    #[test]
    fn normal_of_plane_is_constant_everywhere() {
        let p = Shape::new(ShapeType::Plane);

        let n1 = p.normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_ray_parallel() {
        let p = Shape::new(ShapeType::Plane);
        let r = ray(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));

        let xs = p.intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = Shape::new(ShapeType::Plane);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

        let xs = p.intersect(&r);

        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Shape::new(ShapeType::Plane);
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));

        let xs = p.intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }

    #[test]
    fn ray_intersecting_plane_from_below() {
        let p = Shape::new(ShapeType::Plane);
        let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));

        let xs = p.intersect(&r);

        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p);
    }
}
