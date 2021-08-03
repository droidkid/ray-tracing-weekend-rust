use crate::ray::Ray;
use crate::vec3::dot;
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

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = 2.0 * dot(&oc, ray.direction());
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - 4.0 * a * c;

        fn build_hit_record(ray: &Ray, t: f64, outward_normal: Vec3) -> HitRecord {
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
                ))
            } else if t2 > t_min && t2 < t_max {
                Some(build_hit_record(
                    ray,
                    t2,
                    (ray.at(t2) - self.center).unit_vector(),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
