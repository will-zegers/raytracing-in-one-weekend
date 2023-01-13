use std::fs;

mod camera;
mod color;
mod config;
mod hittable;
mod hittable_list;
mod ray;
mod raytracer;
mod sphere;
mod vec3;

use crate::config::OUTPUT_FILE;
use crate::hittable_list::HittableList;
use crate::raytracer::Raytracer;
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let image = Raytracer::run(&world);

    fs::write(OUTPUT_FILE, image).expect("Unable to write file.");
}
