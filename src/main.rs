mod canvas;
mod color;
mod intersections;
mod light;
mod materials;
mod matrix;
mod rays;
mod sphere;
mod transformations;
mod tuple;

use std::{fs::File, io::Write};

use canvas::Canvas;
use color::Color;
use intersections::hit;
use rays::{intersect, ray};
use sphere::sphere;
use std::f32::consts::PI;
use transformations::{rotation_z, scaling, shearing};
use tuple::{normalise, point};

fn main() {
    println!("Hello, world!");
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f32);
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = sphere();
    shape.transform = rotation_z(PI / 4.0) * scaling(0.5, 1.0, 1.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f32);
            let position = point(world_x, world_y, wall_z);
            let r = ray(
                ray_origin.clone(),
                normalise(&(position - ray_origin.clone())),
            );
            let xs = intersect(&shape, &r);
            if hit(&xs).is_some() {
                canvas.write_pixel(x, y, color.clone());
            }
        }
    }
    let ppm = canvas.to_ppm();
    let file = File::create("output.ppm");
    let _ = file.unwrap().write_all(ppm.as_bytes());
}
