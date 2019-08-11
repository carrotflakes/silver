use super::shapes::shape::Shape;
use super::materials::Material;

pub struct Object {
    pub shape: Box<Shape + std::marker::Sync>,
    pub material: Box<Material + std::marker::Sync>,
}
