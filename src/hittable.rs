use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        let is_front_face = r.direction.dot(*outward_normal) < 0.0;
        self.normal = if is_front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn hit_record_new() {
        let hit_record = HitRecord::new();
        assert_eq!(hit_record.p, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.t, 0.0);
    }

    #[test]
    fn set_face_normal() {
        let mut rng = rand::thread_rng();

        let mut hit_record = HitRecord::new();

        let origin = Point3::new(rng.gen(), rng.gen(), rng.gen());
        let direction = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let ray = Ray::new(origin, direction);

        let outward_normal_same = direction;
        hit_record.set_face_normal(&ray, &outward_normal_same);
        assert_eq!(hit_record.normal, -outward_normal_same);

        let outward_normal_opposite = -direction;
        hit_record.set_face_normal(&ray, &outward_normal_opposite);
        assert_eq!(hit_record.normal, outward_normal_opposite);
    }
}
