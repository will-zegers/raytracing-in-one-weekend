use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

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
}
