use crate::vec3::Vec3;

pub fn linear_to_gamma(v: &Vec3, gamma_factor: f64) -> Vec3 {
    let f = gamma_factor.recip();
    Vec3::new([v.x().powf(f), v.y().powf(f), v.z().powf(f)])
}

pub fn gamma_to_linear(v: &Vec3, gamma_factor: f64) -> Vec3 {
    Vec3::new([
        v.x().powf(gamma_factor),
        v.y().powf(gamma_factor),
        v.z().powf(gamma_factor),
    ])
}

#[inline]
pub fn vec3_to_u64(v: &Vec3) -> u64 {
    use std::mem::transmute;
    unsafe {
        transmute::<f64, u64>(v[0])
            ^ (transmute::<f64, u64>(v[1]) << 1)
            ^ (transmute::<f64, u64>(v[2]) << 2)
    }
}
