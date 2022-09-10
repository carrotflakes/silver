use crate::{shapes::Triangle, vec3::Vec3};

pub fn tetrahedron(center: Vec3, size: f64) -> Vec<Triangle> {
    let v0 = Vec3::new([1.0, 1.0, 1.0]) * size + center;
    let v1 = Vec3::new([1.0, -1.0, -1.0]) * size + center;
    let v2 = Vec3::new([-1.0, 1.0, -1.0]) * size + center;
    let v3 = Vec3::new([-1.0, -1.0, 1.0]) * size + center;
    vec![
        Triangle::new(v0, v1, v2),
        Triangle::new(v0, v3, v1),
        Triangle::new(v0, v2, v3),
        Triangle::new(v1, v3, v2),
    ]
}

pub fn cube(center: Vec3, size: Vec3) -> Vec<Triangle> {
    let v0 = Vec3::new([1.0, 1.0, -1.0]) * size + center;
    let v1 = Vec3::new([1.0, -1.0, -1.0]) * size + center;
    let v2 = Vec3::new([1.0, 1.0, 1.0]) * size + center;
    let v3 = Vec3::new([1.0, -1.0, 1.0]) * size + center;
    let v4 = Vec3::new([-1.0, 1.0, -1.0]) * size + center;
    let v5 = Vec3::new([-1.0, -1.0, -1.0]) * size + center;
    let v6 = Vec3::new([-1.0, 1.0, 1.0]) * size + center;
    let v7 = Vec3::new([-1.0, -1.0, 1.0]) * size + center;
    vec![
        Triangle::new(v0, v4, v6),
        Triangle::new(v0, v6, v2),
        Triangle::new(v3, v2, v6),
        Triangle::new(v3, v6, v7),
        Triangle::new(v7, v6, v4),
        Triangle::new(v7, v4, v5),
        Triangle::new(v5, v1, v3),
        Triangle::new(v5, v3, v7),
        Triangle::new(v1, v0, v2),
        Triangle::new(v1, v2, v3),
        Triangle::new(v5, v4, v0),
        Triangle::new(v5, v0, v1),
    ]
}
