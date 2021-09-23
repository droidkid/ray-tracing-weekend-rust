use crate::geometry::ray::Ray;
use crate::geometry::vec3::{dot, Vec3};
use crate::material::color::Color;
use crate::material::material::{Material, ScatterResult};
use rand::Rng;
use crate::hittable::hittable::HitRecord;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
            let cos = (dot(&(uv * -1.0), normal)).min(1.0);
            let r_out_perp = etai_over_etat * (uv + cos * normal);
            let r_out_parallel = -1.0 * ((1.0 - r_out_perp.len_squared()).sqrt()) * normal;
            r_out_perp + r_out_parallel
        }

        fn reflectance(cos: f64, ref_idx: f64) -> f64 {
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0 * r0;
            return r0 + (1.0 - r0) * ((1.0 - cos).powf(5.0));
        }

        let unit_direction = ray_in.direction().normalize();
        let cos = (dot(&(unit_direction * -1.0), &hit_record.normal)).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = (refraction_ratio * sin) > 1.0;
        let direction;

        if cannot_refract || reflectance(cos, refraction_ratio) > random_double() {
            direction = unit_direction.reflect(&hit_record.normal);
        } else {
            direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }
        ScatterResult {
            scattered_ray: Some(Ray::new(hit_record.hit_point, direction)),
            attenuation: Color::white(),
            emitted: Color::black(),
        }
    }
}

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
