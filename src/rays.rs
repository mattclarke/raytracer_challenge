use crate::tuple::Tuple;

struct Ray {
    origin: Tuple,
    direction: Tuple,
}

fn ray(origin: Tuple, direction: Tuple) -> Ray {
    Ray { origin, direction }
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
}
