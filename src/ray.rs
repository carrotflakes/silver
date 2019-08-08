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

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[test]
fn test_at() {
    let ray: Ray = Ray(Vec3(0.0, 0.0, 0.0), Vec3(1.0, 2.0, 3.0));
    assert_eq!(ray.at(1.0), Vec3(1.0, 2.0, 3.0));
}
