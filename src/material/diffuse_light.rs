use crate::material::color::Color;
use crate::hittable::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;

pub struct DiffuseLight {
    emit_color: Color,
}

impl DiffuseLight {
    pub fn new(emit_color: Color) -> DiffuseLight {
        DiffuseLight { emit_color }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &HitRecord) -> ScatterResult {
        ScatterResult {
            scattered_ray: None,
            attenuation: Color::black(),
            emitted: self.emit_color,
        }
    }
}
