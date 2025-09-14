use crate::{
    intersections::{intersection, Intersection},
    matrix::{inverse, Matrix},
    sphere::Sphere,
    tuple::{dot, point, Tuple},
};

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}

pub fn position(ray: &Ray, t: f32) -> Tuple {
    Tuple {
        x: ray.origin.x + ray.direction.x * t,
        y: ray.origin.y + ray.direction.y * t,
        z: ray.origin.z + ray.direction.z * t,
        w: ray.origin.w + ray.direction.w * t,
    }
}

fn transform(ray: &Ray, m: &Matrix) -> Ray {
    Ray {
        origin: m * &ray.origin,
        direction: m * &ray.direction,
    }
}

pub fn intersect<'a>(s: &'a Sphere, r: &'a Ray) -> Vec<Intersection<'a>> {
    let r = transform(&r, &inverse(&s.transform));

    let sphere_to_ray = r.origin.clone() - point(0.0, 0.0, 0.0);
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&r.direction, &sphere_to_ray);
    let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return vec![];
    }

    let t1 = (-b - f32::sqrt(discriminant)) / (2.0 * a);
    let t2 = (-b + f32::sqrt(discriminant)) / (2.0 * a);

    vec![intersection(t1, &s), intersection(t2, &s)]
}

#[cfg(test)]
mod tests {
    use crate::{
        sphere::sphere,
        transformations::{scaling, translation},
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn create_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let ray = ray(origin.clone(), direction.clone());
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn create_a_point_from_a_distance() {
        let ray = ray(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(position(&ray, 0.0), point(2.0, 3.0, 4.0));
        assert_eq!(position(&ray, 1.0), point(3.0, 3.0, 4.0));
        assert_eq!(position(&ray, -1.0), point(1.0, 3.0, 4.0));
        assert_eq!(position(&ray, 2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn sphere_is_behind_a_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn translating_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
