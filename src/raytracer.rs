use std::fmt::Write;

use rand::prelude::*;

use crate::camera::Camera;
use crate::color::get_pixel_color;
use crate::config::{IMAGE_HEIGHT, IMAGE_WIDTH, SAMPLES_PER_PIXEL};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Color;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_record = HitRecord::new();
    if world.hit(r, 0.0, std::f64::INFINITY, &mut hit_record) {
        return 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub struct Raytracer {}

impl Raytracer {
    pub fn run(world: &dyn Hittable) -> String {
        let camera = Camera::new();

        let mut rng = rand::thread_rng();

        // Render
        let mut image = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in (0..IMAGE_HEIGHT).rev() {
            print!("\rScanlines remaining: {}", j);
            for i in 0..IMAGE_WIDTH {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = ((i as f64) + rng.gen::<f64>()) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + rng.gen::<f64>()) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = camera.get_ray(u, v);
                    color += ray_color(&r, world);
                }
                let pixel_color = get_pixel_color(color, SAMPLES_PER_PIXEL);

                write!(image, "{pixel_color}").unwrap();
            }
        }

        image
    }
}
