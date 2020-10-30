use super::materials::material::Material;
use super::shapes::shape::Shape;

pub struct Object<S: Shape, M: Material> {
    pub shape: S,
    pub material: M,
}
