use crate::{
    canvas::Canvas,
    matrix::{inverse, Matrix},
    rays::{ray, Ray},
    tuple::{normalise, point, Tuple},
    world::{color_at, World},
};

pub struct Camera {
    hsize: u32,
    vsize: u32,
    field_of_view: f32,
    transform: Matrix,
    half_width: f32,
    half_height: f32,
    pixel_size: f32,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f32) -> Camera {
        let (half_width, half_height, pixel_size) =
            Camera::calculate_pixel_size(hsize, vsize, field_of_view);
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_4x4(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    fn calculate_pixel_size(hsize: u32, vsize: u32, field_of_view: f32) -> (f32, f32, f32) {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f32 / vsize as f32;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f32;
        (half_width, half_height, pixel_size)
    }
}

// TODO: move onto Camera
pub fn ray_for_pixel(camera: &Camera, px: u32, py: u32) -> Ray {
    let xoffset = (px as f32 + 0.5) * camera.pixel_size;
    let yoffset = (py as f32 + 0.5) * camera.pixel_size;
    let world_x = camera.half_width - xoffset;
    let world_y = camera.half_height - yoffset;
    let pixel = inverse(&camera.transform) * point(world_x, world_y, -1.0);
    let origin = inverse(&camera.transform) * point(0.0, 0.0, 0.0);
    let direction = normalise(&(&pixel - &origin));
    ray(origin, direction)
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut image = Canvas::new(camera.hsize as usize, camera.vsize as usize);

    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = ray_for_pixel(&camera, x, y);
            let color = color_at(&world, &ray);
            image.write_pixel(x as usize, y as usize, color);
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::{
        color::{self, Color},
        transformations::{rotation_y, translation, view_transform},
        tuple::{point, vector},
        world::World,
    };

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;
        let c = Camera::new(hsize, vsize, fov);
        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.field_of_view, fov);
        assert_eq!(c.transform, Matrix::identity_4x4());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_ray_through_centre_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_a_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = ray_for_pixel(&c, 0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
        let r = ray_for_pixel(&c, 100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            vector(2.0_f32.sqrt() / 2.0, 0.0, -2.0_f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn render_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transform(&from, &to, &up);
        let image = render(&c, &w);
        assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}
