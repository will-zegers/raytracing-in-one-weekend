use crate::vec3::Vec3;

const MAX_COLOR: f64 = 256.0 - std::f64::EPSILON;

pub fn get_pixel_color(pixel: Vec3) -> String {
    format!(
        "{} {} {}\n",
        (MAX_COLOR * pixel.x) as i32,
        (MAX_COLOR * pixel.y) as i32,
        (MAX_COLOR * pixel.z) as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pixel_color() {
        let v = Vec3::new(0.1640625, 0.0, 0.26953125);
        let pixel_color = super::get_pixel_color(v);
        assert_eq!(pixel_color, "42 0 69\n");
    }
}
