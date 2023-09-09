use std::ops::Deref;

use rand::Rng;

use crate::materials::Material;
use crate::pdf::Pdf;
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

    if let Some((hit_rec, material)) = hit.hit(ray) {
        let HitRec {
            location,
            normal,
            uv,
            ..
        } = hit_rec;

        let r = material.ray(&ray, &location, &normal, uv);
        if let Some(scattered) = &r.scattered {
            r.emit + r.albedo * sample(hit, env, scattered, cutoff - 1)
        } else {
            r.emit
        }
    } else {
        env(ray)
    }
}

/// Importance sampling
pub fn sample_weighted<M: Material, DM: Deref<Target = M>>(
    hit: impl Hit<DM>,
    env: impl Fn(&Ray) -> Vec3,
    ray: &Ray,
    cutoff: i32,
    light: &impl crate::shapes::Shape,
) -> Vec3 {
    if cutoff == 0 {
        return Vec3::ZERO;
    }

    if let Some((hit_rec, material)) = hit.hit(ray) {
        let HitRec {
            location,
            normal,
            uv,
            ..
        } = hit_rec;

        let r = material.ray(&ray, &location, &normal, uv);
        if let Some(scattered) = &r.scattered {
            let Some(p1) = r.pdf else {
                return r.albedo * sample_weighted(hit, env, scattered, cutoff - 1, light);
            };
            let p2 = crate::pdf::ShapePdf::new(location, light);
            let p = crate::pdf::MixturePdf::new(&p1, &p2);

            let scattered = Ray::new(location, p.generate());
            let scattering_pdf = material.scattering_pdf(&ray, &normal, &scattered);
            let pdf_value = p.value(&scattered.direction);
            if pdf_value <= 0.0 {
                return r.emit;
            }
            let pdf = scattering_pdf / pdf_value;
            r.emit + r.albedo * sample_weighted(hit, env, &scattered, cutoff - 1, light) * pdf
        } else {
            r.emit
        }
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
                    rng::with(|rng| *Vec3::random_unit_vector(rng)),
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
            if let Some(scattered) = &r.scattered {
                let volume = volume.map(|(d, n, c)| (d - time * ray.direction.norm(), n, c));
                r.emit + r.albedo * sample_with_volume(hit, env, scattered, cutoff - 1, volume)
            } else {
                r.emit
            }
        }
    } else {
        if let Some((scatter_distance, neg_inv_density, color)) = volume {
            // subsurface scattering
            let ray = Ray::new(
                ray.origin + *ray.direction.normalize() * scatter_distance,
                rng::with(|rng| *Vec3::random_unit_vector(rng)),
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
