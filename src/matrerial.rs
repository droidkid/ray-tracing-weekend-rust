use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Vec3,
}

pub struct Metal {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let scatter_direction = hit_record.normal + random_in_unit_sphere();
        Some(ScatterResult {
            ray: Ray::from_to(hit_record.hit_point, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        let unit_vector = ray_in.direction().unit_vector();
        let scatter_direction = unit_vector - 2.0 * dot(&unit_vector, &hit_record.normal) * hit_record.normal;
        let scattered_ray = Ray::from_to(hit_record.hit_point, scatter_direction);
        if dot(scattered_ray.direction(), &hit_record.normal) > 0.0 {
            Some(ScatterResult {
                ray: scattered_ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}
