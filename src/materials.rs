use crate::{color::Color, light::PointLight, tuple::Tuple};

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn default() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eye_vec: &Tuple,
    normal_vec: &Tuple,
) -> Color {
    Color::new(0.0, 0.0, 0.0)
}

#[cfg(test)]
mod tests {
    use crate::{
        color::{self, Color},
        light::PointLight,
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
        let result = lighting(&m, &light, &position, &eye_vec, &normal_vec);
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 + 0.9;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_with_eye_offset_45() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_vec, &normal_vec);
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
        let result = lighting(&m, &light, &position, &eye_vec, &normal_vec);
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 * 2.0_f32.sqrt() / 2.0 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_vec, &normal_vec);
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.9 * 2.0_f32.sqrt() / 2.0 + 0.9;
        assert_eq!(result, Color::new(expected, expected, expected));
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eye_vec = vector(0.0, 0.0, -1.0);
        let normal_vec = vector(0.0, 0.0, -1.0);
        let light = PointLight::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_vec, &normal_vec);
        // Ambient + diffuse + specular
        let expected = 0.1 + 0.0 + 0.0;
        assert_eq!(result, Color::new(expected, expected, expected));
    }
}
