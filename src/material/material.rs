use crate::material::color::Color;
use crate::hittable::HitRecord;
use crate::material::lambertian::Lambertian;
use crate::ray::Ray;
use crate::material::texture::{SolidColorTexture, Texture};
use crate::vec3::{dot, Vec3};
use rand::Rng;

pub struct ScatterResult {
    pub scattered_ray: Option<Ray>,
    pub attenuation: Color,
    pub emitted: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult;
}
