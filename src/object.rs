use crate::shapes::shape::Shape;
use crate::{bbox::BBox, materials::material::Material};

pub struct Object<S: Shape, M: Material> {
    pub(crate) shape: S,
    pub(crate) material: M,
    pub(crate) bbox: BBox,
}

impl<S: Shape, M: Material> Object<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Object {
            bbox: shape.bbox(),
            shape,
            material,
        }
    }
}
