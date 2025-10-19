use crate::{
    color::Color,
    materials::Material,
    tuple::{dot, normalise, reflect, Tuple},
};

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eye_vec: &Tuple,
    normal_vec: &Tuple,
    in_shadow: bool,
) -> Color {
    // Combine surface color with the light's color/intensity.
    let effective_color = &material.color * &light.intensity;

    // Find the direction to the light source.
    let light_vec = normalise(&(&light.position - point));

    // Compute the ambient contribution.
    let ambient = &effective_color * material.ambient;

    // Represents the cosine of the angle between the light vector and the normal vector.
    // Negative means the light is on the other side of the surface.
    let light_dot_normal = dot(&light_vec, &normal_vec);

    let mut diffuse = Color::black();
    let mut specular = Color::black();

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // Represents the cosine of the angle between the reflection vector and the eye vector.
        // Negative means the light reflects away from the eye.
        let temp = -light_vec;
        let reflect_vec = reflect(&temp, &normal_vec);
        let reflect_dot_eye = dot(&reflect_vec, &eye_vec);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = &light.intensity * material.specular * factor;
        }
    }

    if in_shadow {
        ambient
    } else {
        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, tuple::point};

    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = PointLight::new(position.clone(), intensity.clone());
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
