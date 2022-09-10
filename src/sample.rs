use std::ops::Deref;

use crate::materials::Material;
use crate::ray::Ray;
use crate::shapes::HitRec;
use crate::vec3::Vec3;

pub fn sample<M: Material, DM: Deref<Target = M>>(
    hit: impl Fn(&Ray) -> Option<(HitRec, DM)>,
    env: impl Fn(&Ray) -> Vec3,
    ray: &Ray,
    cutoff: i32,
) -> Vec3 {
    if cutoff == 0 {
        return Vec3::ZERO;
    }

    if let Some((
        HitRec {
            location,
            normal,
            uv,
            ..
        },
        material,
    )) = hit(ray)
    {
        let r = material.ray(&ray, &location, &normal, uv);
        let color = sample(hit, env, &r, cutoff - 1);
        material.color(&color, uv)
    } else {
        env(ray)
    }
}
