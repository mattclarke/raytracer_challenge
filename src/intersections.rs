use crate::sphere::Sphere;

struct Intersection<'a> {
    t: f32,
    object: &'a Sphere,
}

fn intersection<'a>(t: f32, object: &'a Sphere) -> Intersection {
    Intersection { t, object }
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
}
