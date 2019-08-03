use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(o: Vec3, d: Vec3) -> Ray {
        Ray {origin: o, direction: d}
    }

    pub fn at(&self, t: f64) -> Ray {
        Ray {origin: self.origin + t * self.direction, direction: self.direction}
    }
}

#[test]
fn test_at() {
    let ray: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0));
    let advanced_ray: Ray = ray.at(1.0);
    assert_eq!(advanced_ray.origin, Vec3::new(1.0, 2.0, 3.0));
}
