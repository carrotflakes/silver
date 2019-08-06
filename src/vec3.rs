use std::ops::{Add, Sub, Mul, Div};
use std::f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub elements: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {elements: [x, y, z]}
    }

    pub const ZERO: Vec3 = Vec3 {elements: [0.0, 0.0, 0.0]};

    pub fn x(&self) -> f64 {
        self.elements[0]
    }

    pub fn y(&self) -> f64 {
        self.elements[1]
    }

    pub fn z(&self) -> f64 {
        self.elements[2]
    }

    pub fn r(&self) -> f64 {
        self.elements[0]
    }

    pub fn g(&self) -> f64 {
        self.elements[1]
    }

    pub fn b(&self) -> f64 {
        self.elements[2]
    }

    pub fn print(&self) -> () {
        println!("{:?}", self);
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {elements: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {elements: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]}
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {elements: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()]}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {elements: [self.x() * rhs, self.y() * rhs, self.z() * rhs]}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {elements: [self * rhs.x(), self * rhs.y(), self * rhs.z()]}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        if rhs == 0.0 {
            return Vec3 {elements: [f64::MAX, f64::MAX, f64::MAX]};
        }
        Vec3 {elements: [self.x() / rhs, self.y() / rhs, self.z() / rhs]}
    }
}

impl Vec3 {
    pub fn norm(&self) -> f64 {
        (self.x().powi(2i32) + self.y().powi(2i32) + self.z().powi(2i32)).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        let norm = self.norm();
        Vec3::new(self.x() / norm, self.y() / norm, self.z() / norm) // panicable!
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x())
    }

    pub fn hadamard(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z())
    }
}

#[test]
fn test_gen() {
    let vec: Vec3 = Vec3::new(0.2, 0.4, 0.8);
    vec.print();
}

#[test]
fn test_norm() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 2.0);
    assert_eq!(vec.norm(), 3.0);
}
