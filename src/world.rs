use crate::{
    color::Color,
    intersections::{hit, prepare_computations, Computations, Intersection},
    light::{lighting, PointLight},
    materials::Material,
    rays::{intersect, ray, Ray},
    shape::Shape,
    sphere::sphere,
    transformations::scaling,
    tuple::{magnitude, normalise, point, Tuple},
};

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn default() -> World {
        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let m = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let mut s1 = sphere();
        s1.set_material(m);
        let mut s2 = sphere();
        s2.set_transform(scaling(0.5, 0.5, 0.5));
        World {
            light,
            objects: vec![s1, s2],
        }
    }
}

pub fn intersect_world(w: &World, r: &Ray) -> Vec<Intersection> {
    let mut result = vec![];
    for o in &w.objects {
        let temp = intersect(&o, &r);
        result.extend_from_slice(&temp);
    }
    result.sort_by(|a, b| a.t.total_cmp(&b.t));
    result
}

pub fn shade_hit(w: &World, comps: &Computations) -> Color {
    let shadowed = is_shadowed(&w, &comps.over_point);
    lighting(
        &comps.object.material(),
        &w.light,
        &comps.over_point,
        &comps.eyev,
        &comps.normalv,
        shadowed,
    )
}

pub fn color_at(w: &World, r: &Ray) -> Color {
    let xs = intersect_world(&w, &r);
    let i = hit(&xs);
    match i {
        Some(i) => {
            let comps = prepare_computations(&i, &r);
            shade_hit(&w, &comps)
        }
        None => Color::new(0.0, 0.0, 0.0),
    }
}

pub fn is_shadowed(world: &World, point: &Tuple) -> bool {
    let v = &world.light.position - point;
    let distance = magnitude(&v);
    let direction = normalise(&v);
    let r = ray(point.clone(), direction);
    let intersections = intersect_world(&world, &r);
    let h = hit(&intersections);
    let Some(h) = h else {
        return false;
    };

    h.t < distance
}

#[cfg(test)]
mod tests {

    use crate::{
        color::Color,
        intersections::{intersection, prepare_computations},
        rays::ray,
        transformations::translation,
        tuple::vector,
    };

    use super::*;

    #[test]
    fn default_world() {
        let light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let m = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let mut s1 = sphere();
        s1.set_material(m);
        let mut s2 = sphere();
        s2.set_transform(scaling(0.5, 0.5, 0.5));

        let w = World::default();
        assert_eq!(w.light, light);
        assert_eq!(w.objects[0].material(), s1.material());
        assert_eq!(w.objects[0].transform(), s1.transform());
        assert_eq!(w.objects[1].material(), s2.material());
        assert_eq!(w.objects[1].transform(), s2.transform());
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = intersect_world(&w, &r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = &w.objects[0];
        let i = intersection(4.0, s);
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = World::default();
        w.light = PointLight::new(point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = &w.objects[1];
        let i = intersection(0.5, s);
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = color_at(&w, &r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_when_ray_hits() {
        let w = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = color_at(&w, &r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let outer = &mut w.objects[0];
        let material = Material::default().ambient(1.0);
        outer.set_material(material.clone());
        let inner = &mut w.objects[1];
        inner.set_material(material);
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = color_at(&w, &r);
        let inner = &w.objects[1];
        assert_eq!(c, inner.material().color);
    }

    #[test]
    fn no_shadow_when_nothing_between_point_and_light() {
        let w = World::default();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(is_shadowed(&w, &p), true);
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(is_shadowed(&w, &p), false);
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let s1 = sphere();
        let mut s2 = sphere();
        s2.set_transform(translation(0.0, 0.0, 10.0));
        let w = World {
            light,
            objects: vec![s1, s2],
        };
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, &w.objects[1]);
        let comps = prepare_computations(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
