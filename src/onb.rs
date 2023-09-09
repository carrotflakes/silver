use crate::vec3::{NormVec3, Vec3};

/// An orthonormal basis.
#[derive(Clone, Copy, Debug)]
pub struct Onb(pub [NormVec3; 3]);

impl Onb {
    pub fn u(&self) -> NormVec3 {
        self.0[0]
    }

    pub fn v(&self) -> NormVec3 {
        self.0[1]
    }

    pub fn w(&self) -> NormVec3 {
        self.0[2]
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        *self.u() * a.x() + *self.v() * a.y() + *self.w() * a.z()
    }

    pub fn from_w(w: NormVec3) -> Self {
        let a = if w.x().abs() > 0.9 {
            Vec3::new([0.0, 1.0, 0.0]).normalize()
        } else {
            Vec3::new([1.0, 0.0, 0.0]).normalize()
        };
        let v = w.cross(&a);
        let u = w.cross(&v);
        Onb([u, v, w])
    }
}
