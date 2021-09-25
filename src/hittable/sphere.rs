use crate::geometry::ray::Ray;
use crate::geometry::vec3::{dot, Vec3};
use crate::hittable::bounding_box::BoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use crate::material::material::Material;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    // TODO(chesetti): Make material reference? You might want to share materials?
    pub material: Box<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = 2.0 * dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        fn build_hit_record<'a, 'b>(
            ray: &'b Ray,
            center: Vec3,
            t: f64,
            outward_normal: Vec3,
            material: &'a Box<dyn Material + Send + Sync>,
        ) -> HitRecord<'a> {
            let hitting_front_face = dot(ray.direction(), &outward_normal) < 0.0;

            let p = ray.at(t) - center;
            let theta = (-p.y()).acos();
            let phi = (-p.z()).atan2(p.x()) + PI;

            HitRecord {
                hit_point: ray.at(t),
                normal: if hitting_front_face {
                    outward_normal
                } else {
                    outward_normal * -1.0
                },
                front_face: hitting_front_face,
                t,
                u: phi / (2.0 * PI),
                v: theta / PI,
                material: Arc::new(material),
            }
        }

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) * 0.5 / a;
            let t2 = (-b + discriminant.sqrt()) * 0.5 / a;

            if t1 > t_min && t1 < t_max {
                Some(build_hit_record(
                    ray,
                    self.center,
                    t1,
                    (ray.at(t1) - self.center) * (1.0 / self.radius),
                    &self.material,
                ))
            } else if t2 > t_min && t2 < t_max {
                Some(build_hit_record(
                    ray,
                    self.center,
                    t2,
                    (ray.at(t2) - self.center) * (1.0 / self.radius),
                    &self.material,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            min_point: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max_point: self.center + Vec3::new(self.radius, self.radius, self.radius),
        }
    }
}
