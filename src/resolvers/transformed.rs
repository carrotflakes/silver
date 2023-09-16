use crate::bbox::BBox;
use crate::matrix::Matrix;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::shapes::HitRec;

use super::Hit;

pub struct Transformed<M: Clone, T: Hit<M>> {
    inner: T,
    matrix: Matrix,
    inv_matrix: Matrix,
    _m: std::marker::PhantomData<M>,
}

impl<M: Clone, T: Hit<M>> Transformed<M, T> {
    pub fn new(inner: T, matrix: Matrix) -> Self {
        Self {
            inner,
            inv_matrix: matrix.inverse(),
            matrix,
            _m: std::marker::PhantomData,
        }
    }
}

impl<M: Clone, T: Hit<M>> Hit<M> for Transformed<M, T> {
    #[inline]
    fn hit_with_range(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<(HitRec, M)> {
        let ray = Ray::new(
            self.inv_matrix.apply(&ray.origin),
            self.inv_matrix.apply(&ray.direction),
        );
        self.inner.hit_with_range(&ray, tmin, tmax).map(|(hr, m)| {
            (
                HitRec {
                    time: hr.time,
                    location: self.matrix.apply(&hr.location),
                    normal: Onb::from_uvw(
                        self.matrix.apply(&hr.normal.u()).normalize(),
                        self.matrix.apply(&hr.normal.v()).normalize(),
                        self.matrix.apply(&hr.normal.w()).normalize(),
                    ),
                    uv: hr.uv,
                    front: hr.front,
                },
                m,
            )
        })
    }
}

impl<M: Clone, T: Hit<M> + AsRef<BBox>> AsRef<BBox> for Transformed<M, T> {
    #[inline]
    fn as_ref(&self) -> &BBox {
        self.inner.as_ref()
    }
}

impl<M: Clone, T: Hit<M> + Clone> Clone for Transformed<M, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            matrix: self.matrix.clone(),
            inv_matrix: self.inv_matrix.clone(),
            _m: self._m,
        }
    }
}
