use crate::material::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct HitRecord<'a> {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub material: Arc<&'a Box<dyn Material + Send + Sync>>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
