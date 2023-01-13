use crate::config::ASPECT_RATIO;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    #[inline]
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin;
        Ray::new(self.origin, direction)
    }
}
