use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fz: f64) -> Metal {
        let mut fuzz = fz;

        if fz > 1.0 {
            fuzz = 1.0;
        }
        if fz < 0.0 {
            fuzz = 0.0;
        }

        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let unit_vector = ray_in.direction().unit_vector();
        let scatter_direction =
            unit_vector.reflect(&hit_record.normal) + self.fuzz * random_in_unit_sphere();
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);
        if dot(scattered_ray.direction(), &hit_record.normal) > 0.0 {
            ScatterResult {
                scattered_ray: Some(scattered_ray),
                attenuation: self.albedo,
                emitted: Color::black(),
            }
        } else {
            ScatterResult {
                scattered_ray: None,
                attenuation: self.albedo,
                emitted: Color::black(),
            }
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
