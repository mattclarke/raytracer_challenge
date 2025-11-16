mod camera;
mod canvas;
mod color;
mod intersections;
mod light;
mod materials;
mod matrix;
mod patterns;
mod plane;
mod rays;
mod shape;
mod sphere;
mod transformations;
mod tuple;
mod world;

use std::{fs::File, io::Write};

use camera::{render, Camera};
use color::Color;
use light::PointLight;
use materials::Material;
use patterns::Stripe;
use plane::plane;
use sphere::sphere;
use std::f64::consts::PI;
use transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use tuple::{point, vector};
use world::World;

fn main() {
    println!("Generating...");
    let mut floor = plane();
    floor.set_transform(scaling(10.0, 0.01, 10.0));
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    floor.set_material(material);

    let mut back_wall = plane();
    back_wall.set_transform(
        translation(0.0, 0.0, 3.0) * rotation_x(PI / 2.0) * scaling(10.0, 0.1, 10.0),
    );
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.5, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    material.pattern = Some(Stripe::new(Color::white(), Color::black()));
    back_wall.set_material(material);

    let mut middle = sphere();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    let mut material = Material::default();
    material.color = Color::new(0.1, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let mut pattern = Stripe::new(Color::white(), Color::black());
    pattern.transformation = scaling(0.25, 1.0, 1.0);
    material.pattern = Some(pattern);
    middle.set_material(material);

    let mut right = sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    let mut material = Material::default();
    material.color = Color::new(0.5, 1.0, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    right.set_material(material);

    let mut left = sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    left.set_material(material);

    let mut world = World::default();
    world.objects.clear();
    world.light = PointLight::new(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.objects.push(floor);
    world.objects.push(back_wall);
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
