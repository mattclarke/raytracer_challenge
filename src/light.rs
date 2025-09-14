use crate::{color::Color, tuple::Tuple};

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
