use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn new() {
        let mut rng = rand::thread_rng();

        let origin = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let direction = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn at() {
        let mut rng = rand::thread_rng();

        let origin = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let direction = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let ray = Ray::new(origin, direction);

        let t = rng.gen::<f64>();
        let got = ray.at(t);
        let expected = ray.origin + t * ray.direction;
        assert_eq!(got, expected);
    }
}
