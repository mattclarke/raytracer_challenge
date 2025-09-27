use crate::{
    color::Color,
    intersections::{Computations, Intersection},
    light::{lighting, PointLight},
    materials::Material,
    rays::{intersect, Ray},
    sphere::{sphere, Sphere},
    transformations::scaling,
    tuple::point,
};

pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn default() -> World {
        let light = PointLight::new(point(-10.0, -10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let m = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let s1 = sphere().material(m);
        let s2 = sphere().transform(scaling(0.5, 0.5, 0.5));
        World {
            light,
            objects: vec![s1, s2],
        }
    }
}

pub fn intersect_world<'a>(w: &'a World, r: &'a Ray) -> Vec<Intersection<'a>> {
    let mut result = vec![];
    for o in &w.objects {
        let temp = intersect(&o, &r);
        result.extend_from_slice(&temp);
    }
    result.sort_by(|a, b| a.t.total_cmp(&b.t));
    result
}

pub fn shade_hit(w: &World, comps: &Computations) -> Color {
    lighting(
        &comps.object.material,
        &w.light,
        &comps.point,
        &comps.eyev,
        &comps.normalv,
    )
}

#[cfg(test)]
mod tests {

    use crate::{
        intersections::{intersection, prepare_computations},
        rays::ray,
        tuple::vector,
    };

    use super::*;

    #[test]
    fn default_world() {
        let light = PointLight::new(point(-10.0, -10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let m = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let s1 = sphere().material(m);
        let s2 = sphere().transform(scaling(0.5, 0.5, 0.5));

        let w = World::default();
        assert_eq!(w.light, light);
        assert_eq!(w.objects[0].material, s1.material);
        assert_eq!(w.objects[0].transform, s1.transform);
        assert_eq!(w.objects[1].material, s2.material);
        assert_eq!(w.objects[1].transform, s2.transform);
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
}
