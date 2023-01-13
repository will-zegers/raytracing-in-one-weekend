use std::fmt::Write;
use std::fs;

use crate::color::get_pixel_color;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

fn ray_color(r: Ray) -> Vec3 {
    let center = Point::new(0.0, 0.0, -1.0);
    let t = hit_sphere(center, 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - center).unit_vector();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Point, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - (radius * radius);

    let discriminant = (half_b * half_b) - (a * c);
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

pub struct Raytracer{}

impl Raytracer {
    pub fn run() {
        // Image
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const IMAGE_WIDTH: u32 = 400;
        const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
        const OUTPUT_FILE: &str = "image.ppm";

        //Camera
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin: Point = Point::new(0.0, 0.0, 0.0);
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
                let pixel_color = get_pixel_color(ray_color(r));

                write!(image, "{pixel_color}").unwrap();
            }
        }

        fs::write(OUTPUT_FILE, image).expect("Unable to write file.");
    }
}
