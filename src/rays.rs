use crate::tuple::Tuple;

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
}

fn position(ray: &Ray, t: f32) -> Tuple {
    Tuple {
        x: ray.origin.x + ray.direction.x * t,
        y: ray.origin.y + ray.direction.y * t,
        z: ray.origin.z + ray.direction.z * t,
        w: ray.origin.w + ray.direction.w * t,
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, vector};

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
}
