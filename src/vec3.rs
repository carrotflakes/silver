use std::f64;
use std::ops::{Add, Deref, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3([0.0; 3]);

    #[inline]
    pub fn new(es: [f64; 3]) -> Self {
        Vec3(es)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[inline]
    pub fn r(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> f64 {
        self.0[2]
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3([self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3([self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3([self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3([self * rhs.x(), self * rhs.y(), self * rhs.z()])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        if rhs == 0.0 {
            Vec3([f64::MAX, f64::MAX, f64::MAX])
        } else {
            Vec3([self.x() / rhs, self.y() / rhs, self.z() / rhs])
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3([-self.x(), -self.y(), -self.z()])
    }
}

impl Vec3 {
    pub fn norm(&self) -> f64 {
        (self.x().powi(2i32) + self.y().powi(2i32) + self.z().powi(2i32)).sqrt()
    }

    pub fn normalize(&self) -> NormVec3 {
        let inv_norm = 1.0 / self.norm(); // panicable!
        NormVec3(Vec3([
            self.x() * inv_norm,
            self.y() * inv_norm,
            self.z() * inv_norm,
        ]))
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3([
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        ])
    }

    pub fn hadamard(&self, rhs: &Vec3) -> Vec3 {
        Vec3([self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()])
    }

    pub fn random(rng: &mut impl rand::Rng) -> Vec3 {
        Vec3([rng.gen(), rng.gen(), rng.gen()])
    }

    #[inline]
    pub fn random_in_unit_sphere(rng: &mut impl rand::Rng) -> Vec3 {
        loop {
            let v = Vec3([
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
                rng.gen::<f64>() * 2.0 - 1.0,
            ]);
            if v.dot(&v) < 1.0 {
                return v;
            }
        }
    }

    #[inline]
    pub fn random_on_unit_sphere(rng: &mut impl rand::Rng) -> NormVec3 {
        use std::f64::consts::TAU;
        let r1 = rng.gen::<f64>();
        let r2 = rng.gen::<f64>();
        let x = (TAU * r1).cos() * 2.0 * r2 * (1.0 - r2).sqrt();
        let y = (TAU * r1).sin() * 2.0 * (r2 * (1.0 - r2)).sqrt();
        let z = 1.0 - 2.0 * r2;
        NormVec3(Vec3([x, y, z]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormVec3(Vec3);

impl Deref for NormVec3 {
    type Target = Vec3;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
