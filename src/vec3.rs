use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn as_unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        (1.0 / t) * self
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    const EPSILON: f64 = 1000.0 * std::f64::EPSILON;

    #[test]
    fn new() {
        let mut rng = rand::thread_rng();

        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        let v = Vec3::new(x, y, z);

        assert_eq!(v.x, x);
        assert_eq!(v.y, y);
        assert_eq!(v.z, z);
    }

    #[test]
    fn add() {
        let mut rng = rand::thread_rng();

        let v1 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v2 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v3 = v1 + v2;

        assert_eq!(v3.x, v1.x + v2.x);
        assert_eq!(v3.y, v1.y + v2.y);
        assert_eq!(v3.z, v1.z + v2.z);
    }

    #[test]
    fn add_assign() {
        let mut rng = rand::thread_rng();

        let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let mut v1 = v;
        let v2 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        v1 += v2;

        assert_eq!(v1.x, v.x + v2.x);
        assert_eq!(v1.y, v.y + v2.y);
        assert_eq!(v1.z, v.z + v2.z);
    }

    #[test]
    fn neg() {
        let mut rng = rand::thread_rng();

        let v = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v_neg = -v;

        assert_eq!(v_neg.x, -v.x);
        assert_eq!(v_neg.y, -v.y);
        assert_eq!(v_neg.z, -v.z);
    }

    #[test]
    fn f64_mul_vec3() {
        let mut rng = rand::thread_rng();

        let t = rng.gen::<f64>();
        let v1 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v2 = t * v1;

        assert_eq!(v2.x, t * v1.x);
        assert_eq!(v2.y, t * v1.y);
        assert_eq!(v2.z, t * v1.z);
    }

    #[test]
    fn vec3_div_f64() {
        let mut rng = rand::thread_rng();

        let t = rng.gen::<f64>();
        let v1 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v2 = v1 / t;

        assert!((v2.x - v1.x / t).abs() < EPSILON);
        assert!((v2.y - v1.y / t).abs() < EPSILON);
        assert!((v2.z - v1.z / t).abs() < EPSILON);
    }

    #[test]
    fn length() {
        let mut rng = rand::thread_rng();

        let x = rng.gen();
        let y = rng.gen();
        let z = rng.gen();
        let v = Vec3::new(x, y, z);

        let expected = ((x * x) + (y * y) + (z * z)).sqrt();
        assert_eq!(v.length(), expected);
    }

    #[test]
    fn sub() {
        let mut rng = rand::thread_rng();

        let v1 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v2 = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let v3 = v1 - v2;

        assert_eq!(v3.x, v1.x - v2.x);
        assert_eq!(v3.y, v1.y - v2.y);
        assert_eq!(v3.z, v1.z - v2.z);
    }
}
