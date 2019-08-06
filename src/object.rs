use super::shapes::shape::Shape;
use super::materials::Material;

pub struct Object {
    pub shape: Box<Shape>,
    pub material: Box<Material>,
}
