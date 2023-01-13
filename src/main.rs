mod color;
mod hittable;
mod ray;
mod raytracer;
mod sphere;
mod vec3;

use crate::raytracer::Raytracer;

fn main() {
    Raytracer::run();
}
