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

use camera::{render, Camera};
use canvas::Canvas;
use color::Color;
use intersections::hit;
use light::{lighting, PointLight};
use materials::Material;
use rays::{intersect, position, ray};
use sphere::{normal_at, sphere};
use std::f32::consts::PI;
use transformations::{
    rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform,
};
use tuple::{normalise, point, vector};
use world::World;

fn main() {
    println!("Generating...");
    let mut floor = sphere();
    floor.transform = scaling(10.0, 0.01, 10.0);
    floor.material = Material::default();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = sphere();
    left_wall.transform = translation(0.0, 0.0, 5.0)
        * rotation_y(-PI / 4.0)
        * rotation_x(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    left_wall.material = Material::default();
    left_wall.material.color = Color::new(1.0, 0.9, 0.9);
    left_wall.material.specular = 0.0;

    let mut right_wall = sphere();
    right_wall.transform = translation(0.0, 0.0, 5.0)
        * rotation_y(PI / 4.0)
        * rotation_x(PI / 2.0)
        * scaling(10.0, 0.01, 10.0);
    right_wall.material = Material::default();
    right_wall.material.color = Color::new(1.0, 0.9, 0.9);
    right_wall.material.specular = 0.0;

    let mut middle = sphere();
    middle.transform = translation(-0.5, 1.0, 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = sphere();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = sphere();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    left.material = Material::default();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::default();
    world.objects.clear();
    world.light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(right);
    world.objects.push(left);

    let mut camera = Camera::new(300, 150, PI / 3.0);
    camera.transform = view_transform(
        &point(0.0, 1.5, -5.0),
        &point(0.0, 1.0, 0.0),
        &vector(0.0, 1.0, 0.0),
    );

    let canvas = render(&camera, &world);
    let ppm = canvas.to_ppm();
    let file = File::create("output.ppm");
    let _ = file.unwrap().write_all(ppm.as_bytes());
}
