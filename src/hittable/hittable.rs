use crate::geometry::ray::Ray;
use crate::geometry::vec3::Vec3;
use crate::hittable::bounding_box::BoundingBox;
use crate::material::material::Material;
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

    fn get_bounding_box(&self) -> BoundingBox;
}
