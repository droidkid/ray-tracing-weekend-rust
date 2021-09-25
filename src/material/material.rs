use crate::geometry::ray::Ray;
use crate::geometry::vec3::{dot, Vec3};
use crate::hittable::hittable::HitRecord;
use crate::material::color::Color;
use crate::material::lambertian::Lambertian;
use crate::material::texture::Texture;
use rand::Rng;

pub struct ScatterResult {
    pub scattered_ray: Option<Ray>,
    pub attenuation: Color,
    pub emitted: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult;
}
