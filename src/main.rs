mod camera;
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
mod world;

use std::{fs::File, io::Write};

use canvas::Canvas;
use color::Color;
use intersections::hit;
use light::{lighting, PointLight};
use materials::Material;
use rays::{intersect, position, ray};
use sphere::{normal_at, sphere};
use std::f32::consts::PI;
use transformations::{rotation_z, scaling, shearing};
use tuple::{normalise, point};

fn main() {
    println!("Generating...");
    let ray_origin = point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 300;
    let pixel_size = wall_size / (canvas_pixels as f32);
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);
    let mut shape = sphere();
    shape.material = material;
    //shape.transform = rotation_z(PI / 4.0) * scaling(0.5, 1.0, 1.0);

    let light_source = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f32);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f32);
            let pos = point(world_x, world_y, wall_z);
            let r = ray(ray_origin.clone(), normalise(&(pos - ray_origin.clone())));
            let xs = intersect(&shape, &r);
            if let Some(h) = hit(&xs) {
                let point = position(&r, h.t);
                let normal = normal_at(&h.object, &point);
                let eye = -r.direction.clone();
                let color = lighting(&h.object.material, &light_source, &point, &eye, &normal);
                canvas.write_pixel(x, y, color.clone());
            }
        }
    }
    let ppm = canvas.to_ppm();
    let file = File::create("output.ppm");
    let _ = file.unwrap().write_all(ppm.as_bytes());
}
