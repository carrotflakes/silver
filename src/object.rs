use super::shapes::shape::Shape;
use super::materials::material::Material;

pub struct Object {
    pub shape: Box<Shape + std::marker::Sync>,
    pub material: Box<Material + std::marker::Sync>,
}
