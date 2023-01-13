use crate::vec3::Vec3;

const MIN_SCALE: f64 = 0.0;
const MAX_SCALE: f64 = 1.0 - std::f64::EPSILON;

#[inline]
fn clamp(n: f64) -> f64 {
    return if n < MIN_SCALE {
        MIN_SCALE
    } else if n > MAX_SCALE {
        MAX_SCALE
    } else {
        n
    };
}

pub fn get_pixel_color(pixel: Vec3, sample_count: u32) -> String {
    let scale = 1.0 / (sample_count as f64);
    let r = scale * pixel.x;
    let g = scale * pixel.y;
    let b = scale * pixel.z;

    format!(
        "{} {} {}\n",
        (256.0 * clamp(r)) as u8,
        (256.0 * clamp(g)) as u8,
        (256.0 * clamp(b)) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pixel_color() {
        let v = Vec3::new(0.1640625, 0.0, 0.26953125);
        let pixel_color = super::get_pixel_color(v, 1);
        assert_eq!(pixel_color, "42 0 69\n");

        let v = Vec3::new(16.40625, 0.0, 26.953125);
        let pixel_color = super::get_pixel_color(v, 100);
        assert_eq!(pixel_color, "42 0 69\n");
    }
}
