use super::vec3::Vec3;
use super::ray::Ray;

pub trait Material {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray;
    fn color(&self, coler: &Vec3) -> Vec3;
}

pub struct Metal {
    fuzz: f64,
}

impl Metal {
    pub fn new(fuzz: f64) -> Metal {
        Metal {fuzz: fuzz}
    }
}

impl Material for Metal {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * *normal;
        let f: Vec3 = self.fuzz * Vec3::random_in_unit_sphere();
        Ray::new(*location, ray.direction + 2.0 * b + f)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        *color
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {albedo: albedo}
    }
}

impl Material for Lambertian {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        Ray::new(*location, *normal + Vec3::random_in_unit_sphere())
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        color.hadamard(&self.albedo)
    }
}

pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Dielectric {
        Dielectric {ri: ri}
    }
}

impl Material for Dielectric {
    fn ray(&self, ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        let b: Vec3 = -(ray.direction.dot(normal)) * *normal;
        let reflected: Vec3 = ray.direction + 2.0 * b;

        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(normal) > 0.0 {
            (-*normal, self.ri, self.ri * ray.direction.dot(normal) / ray.direction.norm())
        } else {
            (*normal, self.ri.recip(), -ray.direction.dot(normal) / ray.direction.norm())
        };

        let v = match refract(&-ray.direction, &outward_normal, ni_over_nt) {
            Option::Some(ref refracted)
                if rand::random::<f64>() >= schlick(cosine, self.ri)
                => *refracted,
            _ => reflected
        };
        Ray::new(*location, v)
    }

    fn color(&self, color: &Vec3) -> Vec3 {
        *color
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let d = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if d > 0.0 {
        Option::Some(-ni_over_nt * (uv - *n * dt) - *n * d.sqrt())
    } else {
        Option::None
    }
}

#[inline(always)]
fn schlick(cosine: f64, ri: f64) -> f64 {
    let r0 = (1.0 - ri) / (1.0 + ri).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct DiffuseLight {
    color: Vec3,
}

impl DiffuseLight {
    pub fn new(color: Vec3) -> DiffuseLight {
        DiffuseLight {color: color}
    }
}

impl Material for DiffuseLight {
    fn ray(&self, _ray: &Ray, location: &Vec3, normal: &Vec3) -> Ray {
        Ray::new(*location, *normal)
    }

    fn color(&self, _color: &Vec3) -> Vec3 {
        self.color
    }
}
