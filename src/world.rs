use crate::{
    color::Color,
    light::PointLight,
    materials::Material,
    sphere::{sphere, Sphere},
    transformations::scaling,
    tuple::point,
};

struct World {
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

#[cfg(test)]
mod tests {

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
}
