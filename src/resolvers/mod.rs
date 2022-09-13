pub mod bvh;
pub mod linear_search;
pub mod object;

use crate::{ray::Ray, shapes::HitRec};

pub trait Hit<R> {
    fn hit_with_range(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(HitRec, R)>;
    fn hit(&self, ray: &Ray) -> Option<(HitRec, R)> {
        self.hit_with_range(ray, 1e-6, std::f64::MAX)
    }
}

impl<R, H: Hit<R>, T: std::ops::Deref<Target = H>> Hit<R> for T {
    #[inline]
    fn hit_with_range(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(HitRec, R)> {
        self.deref().hit_with_range(ray, tmin, tmax)
    }
}
