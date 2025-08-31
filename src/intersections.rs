use crate::sphere::Sphere;

#[derive(Clone, Debug, PartialEq)]
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

pub fn hit<'a>(xs: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    let mut result = None;

    for i in xs {
        if i.t <= 0.0 {
            continue;
        }
        if result.is_none() {
            result = Some(i);
        } else if i.t < result.unwrap().t {
            result = Some(i);
        }
    }

    result
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

    #[test]
    fn all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let xs = intersections(vec![i2.clone(), i1.clone()]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(-2.0, &s);
        let xs = intersections(vec![i2, i1]);
        let i = hit(&xs);
        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_lowest_nonegative_intersection() {
        let s = sphere();
        let i1 = intersection(5.0, &s);
        let i2 = intersection(7.0, &s);
        let i3 = intersection(-3.0, &s);
        let i4 = intersection(2.0, &s);
        let xs = intersections(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i4);
    }
}
