use std::fmt::Write;
use std::fs;

mod vec3;

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
            let r = (i as f64) / ((IMAGE_WIDTH as f64) - 1.);
            let g = (j as f64) / ((IMAGE_HEIGHT as f64) - 1.);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            write!(image, "{} {} {}\n", ir, ig, ib).unwrap();
        }
    }

    fs::write(OUTPUT_FILE, image).expect("Unable to write file.");
}
