use crate::{
    rays::{position, Ray},
    sphere::{normal_at, Sphere},
    tuple::{dot, Tuple},
};

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

pub struct Computations<'a> {
    t: f32,
    object: &'a Sphere,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
}

pub fn prepare_computations<'a>(intersection: &'a Intersection, ray: &'a Ray) -> Computations<'a> {
    let point = position(&ray, intersection.t);
    let eyev = -ray.direction.clone();
    let mut normalv = normal_at(intersection.object, &point);
    let inside = if dot(&normalv, &eyev) < 0.0 {
        normalv = -normalv;
        true
    } else {
        false
    };

    Computations {
        t: intersection.t,
        object: intersection.object,
        point: position(&ray, intersection.t),
        eyev,
        normalv,
        inside,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::ray,
        sphere::sphere,
        tuple::{point, vector},
    };

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

    #[test]
    fn precompute_state_of_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let i = intersection(4.0, &s);
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn hit_when_intersection_occurs_on_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let i = intersection(1.0, &s);
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }
}
