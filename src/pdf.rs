use rand::Rng;

use crate::{
    onb::Onb,
    rng,
    shapes::Shape,
    vec3::{NormVec3, Vec3},
};

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: NormVec3) -> Self {
        CosinePdf {
            uvw: Onb::from_w(w),
        }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.normalize().dot(&self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / std::f64::consts::PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw
            .local(rng::with(|rng| *Vec3::random_cosine_direction(rng)))
    }
}

pub struct ShapePdf<'a, S: Shape> {
    origin: Vec3,
    shape: &'a S,
}

impl<'a, S: Shape> ShapePdf<'a, S> {
    pub fn new(origin: Vec3, shape: &'a S) -> Self {
        ShapePdf { origin, shape }
    }
}

impl<'a, S: Shape> Pdf for ShapePdf<'a, S> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.shape.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.shape.random(&self.origin)
    }
}

pub struct MixturePdf<'a, P1: Pdf, P2: Pdf> {
    p1: &'a P1,
    p2: &'a P2,
    ratio: f64,
}

impl<'a, P1: Pdf, P2: Pdf> MixturePdf<'a, P1, P2> {
    pub fn new(p1: &'a P1, p2: &'a P2) -> Self {
        MixturePdf { p1, p2, ratio: 0.2 } // TODO
    }
}

impl<'a, P1: Pdf, P2: Pdf> Pdf for MixturePdf<'a, P1, P2> {
    fn value(&self, direction: &Vec3) -> f64 {
        (1.0 - self.ratio) * self.p1.value(direction) + self.ratio * self.p2.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rng::with(|rng| rng.gen_bool(self.ratio)) {
            self.p2.generate()
        } else {
            self.p1.generate()
        }
    }
}
