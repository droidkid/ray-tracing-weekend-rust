use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub hit_point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
