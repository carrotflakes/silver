pub mod bbox;
pub mod bvh;
pub mod camera;
pub mod envs;
pub mod formats;
pub mod linear_search;
pub mod materials;
pub mod primitives;
pub mod ray;
pub mod render;
pub mod rng;
pub mod sample;
pub mod shapes;
pub mod vec3;

#[inline]
pub fn vec3_to_u64(v: &vec3::Vec3) -> u64 {
    use std::mem::transmute;
    unsafe {
        transmute::<f64, u64>(v[0])
            ^ (transmute::<f64, u64>(v[1]) << 1)
            ^ (transmute::<f64, u64>(v[2]) << 2)
    }
}
