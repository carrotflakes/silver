use crate::{ray::Ray, vec3::Vec3};

pub fn default_env(ray: &Ray) -> Vec3 {
    let direction = ray.direction.normalize();
    let t: f64 = 0.5 * (1.0 - direction.y());
    (1.0 - t) * Vec3::new([1.0, 1.0, 1.0]) + t * Vec3::new([0.5, 0.7, 1.0])
}

pub fn fancy_env(ray: &Ray) -> Vec3 {
    let direction = ray.direction.normalize();
    ((direction.x() * 5.0).sin() * (direction.y() * 5.0).sin() * (direction.z() * 5.0).sin() + 1.0)
        * Vec3::new([0.5, 0.0, 0.0])
        + ((direction.x() * 6.0).cos() * (direction.y() * 6.0).cos() * (direction.z() * 6.0).cos()
            + 1.0)
            * Vec3::new([0.0, 0.5, 0.0])
        + ((direction.x() * 7.0).sin() * (direction.y() * 7.0).sin() * (direction.z() * 7.0).sin()
            + 1.0)
            * Vec3::new([0.0, 0.0, 0.5])
}

pub fn dark_env(_: &Ray) -> Vec3 {
    Vec3::ZERO
}
