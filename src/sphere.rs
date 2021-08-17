use crate::hittable::{HitRecord, Hittable};
use crate::matrerial::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    // TODO(chesetti): Make material reference? You might want to share materials?
    pub material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = 2.0 * dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        fn build_hit_record<'a, 'b>(ray: &'b Ray, t: f64, outward_normal: Vec3, material: &'a Box<dyn Material>) -> HitRecord<'a> {
            let hitting_front_face = dot(ray.direction(), &outward_normal) < 0.0;
            HitRecord {
                hit_point: ray.at(t),
                normal: if hitting_front_face {
                    outward_normal
                } else {
                    outward_normal * -1.0
                },
                front_face: hitting_front_face,
                t,
                material
            }
        }

        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) * 0.5 / a;
            let t2 = (-b + discriminant.sqrt()) * 0.5 / a;

            if t1 > t_min && t1 < t_max {
                Some(build_hit_record(
                    ray,
                    t1,
                    (ray.at(t1) - self.center).unit_vector(),
                    &self.material
                ))
            } else if t2 > t_min && t2 < t_max {
                Some(build_hit_record(
                    ray,
                    t2,
                    (ray.at(t2) - self.center).unit_vector(),
                    &self.material
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
