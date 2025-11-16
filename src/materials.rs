use crate::{color::Color, patterns::Stripe};

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Stripe>,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }

    pub fn color(mut self, c: Color) -> Material {
        self.color = c;
        self
    }

    pub fn ambient(mut self, a: f64) -> Material {
        self.ambient = a;
        self
    }

    pub fn diffuse(mut self, d: f64) -> Material {
        self.diffuse = d;
        self
    }

    pub fn specular(mut self, s: f64) -> Material {
        self.specular = s;
        self
    }

    pub fn shininess(mut self, s: f64) -> Material {
        self.shininess = s;
        self
    }

    pub fn pattern(mut self, p: Stripe) -> Material {
        self.pattern = Some(p);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        light::{lighting, PointLight},
        patterns::Stripe,
        sphere::sphere,
        tuple::{point, vector},
    };

    use super::*;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            false,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 + 0.9;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset_45() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            false,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_with_light_offset_45() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            false,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            false,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 * 2.0_f64.sqrt() / 2.0 + 0.9;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            false,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.0 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &position,
            &eye_vec,
            &normal_vec,
            true,
        );
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.0 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_pattern() {
        let m = Material::default()
            .ambient(1.0)
            .diffuse(0.0)
            .specular(0.0)
            .pattern(Stripe::new(Color::white(), Color::black()));
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &point(0.9, 0.0, 0.0),
            &eye_vec,
            &normal_vec,
            false,
        );
        let c2 = lighting(
            &m,
            &sphere(), // Unused
            &light,
            &point(1.1, 0.0, 0.0),
            &eye_vec,
            &normal_vec,
            false,
        );
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }
}
