use crate::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

pub fn intersection<'a>(t: f32, object: &'a Sphere) -> Intersection<'a> {
    Intersection { t, object }
}

fn intersections(is: Vec<Intersection>) -> Vec<Intersection> {
    is
}

#[cfg(test)]
mod tests {
    use crate::sphere::sphere;

    use super::*;

    #[test]
    fn intersection_has_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(&s, i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let xs = intersections(vec![i1, i2]);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }
}
