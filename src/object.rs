use super::materials::material::Material;
use super::shapes::shape::Shape;

pub struct Object {
    pub shape: Box<dyn Shape + std::marker::Sync>,
    pub material: Box<dyn Material + std::marker::Sync>,
}
