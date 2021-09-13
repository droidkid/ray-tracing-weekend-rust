use crate::vec3::{Vec3, dot};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use std::sync::Arc;

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Send + Sync>,
}

impl Plane {
    pub fn xy_plane(z: f64, material: Box<dyn Material + Send + Sync>) -> Plane {
        Plane {
            point: Vec3::new(0.0, 0.0, z),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let den = dot(ray.direction(), &self.normal);
        if den.abs() < 1e-6 {
            return None;
        }
        let num = dot((&(self.point - ray.origin())), &self.normal);
        let t = num / den;

        if t < t_min || t > t_max {
            return None;
        }

        // Let's do an infinite plane for now.
        let hit_point = ray.at(t);

        Some(HitRecord {
            hit_point,
            normal: if den < 0.0 {
                self.normal
            } else {
                self.normal * -1.0
            },
            front_face: true,
            t,
            u: 0.0, // TODO(): implement
            v: 0.0, // TODO(): implement!
            material: Arc::new(&Box::new(&self.material))
        })

    }
}