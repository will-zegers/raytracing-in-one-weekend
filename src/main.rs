use std::fmt::Write;
use std::fs;

mod color;
use crate::color::get_pixel_color;
mod ray;
mod vec3;
use crate::vec3::Vec3;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = IMAGE_WIDTH;
    const OUTPUT_FILE: &str = "image.ppm";

    // Render
    let mut image = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel = Vec3::new(
                (i as f64) / ((IMAGE_WIDTH as f64) - 1.),
                (j as f64) / ((IMAGE_HEIGHT as f64) - 1.),
                0.25,
            );

            let pixel_color = get_pixel_color(pixel);
            write!(image, "{pixel_color}").unwrap();
        }
    }

    fs::write(OUTPUT_FILE, image).expect("Unable to write file.");
}
