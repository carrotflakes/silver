use std::ops::Deref;

use rand::Rng;

use crate::materials::Material;
use crate::ray::Ray;
use crate::resolvers::Hit;
use crate::rng;
use crate::shapes::HitRec;
use crate::vec3::Vec3;

pub fn sample<M: Material, DM: Deref<Target = M>>(
    hit: impl Hit<DM>,
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
    )) = hit.hit(ray)
    {
        let r = material.ray(&ray, &location, &normal, uv);
        let color = if material.scatter() {
            sample(hit, env, &r, cutoff - 1)
        } else {
            Vec3::ZERO
        };
        material.color(&color, uv)
    } else {
        env(ray)
    }
}

pub fn sample_with_volume<M: Material, DM: Deref<Target = M>>(
    hit: impl Hit<DM>,
    env: impl Fn(&Ray) -> Vec3,
    ray: &Ray,
    cutoff: i32,
    volume: Option<(f64, f64, Vec3)>,
) -> Vec3 {
    if cutoff == 0 {
        return Vec3::ZERO;
    }

    if let Some((
        HitRec {
            location,
            normal,
            uv,
            front,
            time,
            ..
        },
        material,
    )) = hit.hit(ray)
    {
        if let Some((scatter_distance, neg_inv_density, color)) = volume {
            if scatter_distance < time * ray.direction.norm() {
                // subsurface scattering
                let ray = Ray::new(
                    ray.origin + *ray.direction.normalize() * scatter_distance,
                    rng::with(|rng| *Vec3::random_on_unit_sphere(rng)),
                );
                return sample_with_volume(
                    hit,
                    env,
                    &ray,
                    cutoff - 1,
                    Some((
                        make_scatter_distance(neg_inv_density),
                        neg_inv_density,
                        color,
                    )),
                ) * color;
            }
        }
        if let Some((neg_inv_density, color)) = material.volume() {
            if front {
                // into the volume face
                let ray = Ray::new(location, ray.direction);
                sample_with_volume(
                    hit,
                    env,
                    &ray,
                    cutoff,
                    Some((
                        make_scatter_distance(neg_inv_density),
                        neg_inv_density,
                        color,
                    )),
                )
            } else {
                // out of the volume face
                sample_with_volume(hit, env, &Ray::new(location, ray.direction), cutoff, None)
            }
        } else {
            let r = material.ray(&ray, &location, &normal, uv);
            let color = if material.scatter() {
                let volume = volume.map(|(d, n, c)| (d - time * ray.direction.norm(), n, c));
                sample_with_volume(hit, env, &r, cutoff - 1, volume)
            } else {
                Vec3::ZERO
            };
            material.color(&color, uv)
        }
    } else {
        if let Some((scatter_distance, neg_inv_density, color)) = volume {
            // subsurface scattering
            let ray = Ray::new(
                ray.origin + *ray.direction.normalize() * scatter_distance,
                rng::with(|rng| *Vec3::random_on_unit_sphere(rng)),
            );
            return sample_with_volume(
                hit,
                env,
                &ray,
                cutoff - 1,
                Some((
                    make_scatter_distance(neg_inv_density),
                    neg_inv_density,
                    color,
                )),
            ) * color;
        }

        env(ray)
    }
}

pub fn make_scatter_distance(neg_inv_density: f64) -> f64 {
    -neg_inv_density * rng::with(|rng| rng.gen::<f64>()).ln()
}
