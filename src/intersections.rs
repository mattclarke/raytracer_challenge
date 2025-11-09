use crate::{
    matrix::EPSILON,
    rays::{position, Ray},
    shape::Shape,
    tuple::{dot, Tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

pub fn intersection(t: f64, object: &Shape) -> Intersection {
    Intersection {
        t,
        object: object.clone(),
    }
}

fn intersections(is: Vec<Intersection>) -> Vec<Intersection> {
    is
}

pub fn hit(xs: &Vec<Intersection>) -> Option<&Intersection> {
    let mut result = None;

    for i in xs {
        if i.t <= 0.0 {
            continue;
        }

        if result.is_none() {
            result = Some(i);
            continue;
        }

        if i.t < result.unwrap().t {
            result = Some(i);
        }
    }

    result
}

pub struct Computations {
    t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

pub fn prepare_computations(intersection: &Intersection, ray: &Ray) -> Computations {
    let point = position(ray, intersection.t);
    let eyev = -ray.direction.clone();
    let mut normalv = Shape::normal_at(&intersection.object, &point);
    let inside = if dot(&normalv, &eyev) < 0.0 {
        normalv = -normalv;
        true
    } else {
        false
    };

    let point = position(ray, intersection.t);
    let over_point = &point + &(&normalv * EPSILON);

    Computations {
        t: intersection.t,
        object: intersection.object.clone(),
        point,
        over_point,
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
        transformations::translation,
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn intersection_has_t_and_object() {
        let s = sphere();
        let i = intersection(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(s, i.object);
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

    #[test]
    fn hit_should_offset_the_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.set_transform(translation(0.0, 0.0, 1.0));
        let i = intersection(5.0, &s);
        let comps = prepare_computations(&i, &r);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
