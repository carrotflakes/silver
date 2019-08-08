use std::ops::{Add, Sub, Mul, Div, Neg};
use std::f64;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        if rhs == 0.0 {
            Vec3(f64::MAX, f64::MAX, f64::MAX)
        } else {
            Vec3(self.x() / rhs, self.y() / rhs, self.z() / rhs)
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.x(), -self.y(), -self.z())
    }
}

impl Vec3 {
    pub fn norm(&self) -> f64 {
        (self.x().powi(2i32) + self.y().powi(2i32) + self.z().powi(2i32)).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        let norm = self.norm();
        Vec3(self.x() / norm, self.y() / norm, self.z() / norm) // panicable!
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x())
    }

    pub fn hadamard(&self, rhs: &Vec3) -> Vec3 {
        Vec3(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z())
    }

    pub fn random() -> Vec3 {
        Vec3(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3(
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0);
            if v.dot(&v) < 1.0 {
                return v;
            }
        }
    }
}

#[test]
fn test_gen() {
    let vec: Vec3 = Vec3(0.2, 0.4, 0.8);
    vec.print();
}

#[test]
fn test_norm() {
    let vec: Vec3 = Vec3(1.0, 2.0, 2.0);
    assert_eq!(vec.norm(), 3.0);
}
