use std::fmt::Write;
use std::fs;

use crate::color::get_pixel_color;
use crate::config::*;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
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
    pub fn run(world: &dyn Hittable) {
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner: Vec3 =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        // Render
        let mut image = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in (0..IMAGE_HEIGHT).rev() {
            print!("\rScanlines remaining: {}", j);
            for i in 0..IMAGE_WIDTH {
                let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
                let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

                let direction = lower_left_corner + (u * horizontal) + (v * vertical) - origin;
                let r = Ray::new(origin, direction);
                let ray_color = ray_color(&r, world);
                let pixel_color = get_pixel_color(ray_color);

                write!(image, "{pixel_color}").unwrap();
            }
        }

        fs::write(OUTPUT_FILE, image).expect("Unable to write file.");
    }
}
