use crate::geometry::ray::Ray;
use crate::geometry::vec3::{cross, dot, Vec3};
use crate::hittable::bounding_box::AabbBoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use crate::hittable::triangle::Triangle;
use crate::material::color::Color;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::material::Material;
use crate::material::metal::Metal;
use std::sync::Arc;

pub struct Quad {
    triangle1: Triangle,
    triangle2: Triangle,
}

impl Quad {
    pub fn new_lambertian(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3, color: Color) -> Quad {
        // TODO(chesetti): Rotate points in clockwise manner around p1.
        let material: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Lambertian::new_from_color(color)));
        Quad {
            triangle1: Triangle::new(p1, p2, p3, Arc::clone(&material)),
            triangle2: Triangle::new(p1, p3, p4, Arc::clone(&material)),
        }
    }

    pub fn new_diffuse_light(p1: Vec3, p2: Vec3, p3: Vec3, p4: Vec3, color: Color) -> Quad {
        let material: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(DiffuseLight::new(color)));
        Quad {
            triangle1: Triangle::new(p1, p2, p3, Arc::clone(&material)),
            triangle2: Triangle::new(p1, p3, p4,  Arc::clone(&material)),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let hit1 = self.triangle1.hit(ray, t_min, t_max);
        let hit2 = self.triangle2.hit(ray, t_min, t_max);

        if hit1.is_none() && hit2.is_none() {
            return None;
        }

        if hit1.is_none() {
            return hit2;
        }

        if hit2.is_none() {
            return hit1;
        }

        return if hit1.as_ref().unwrap().t < hit2.as_ref().unwrap().t {
            hit1
        } else {
            hit2
        };
    }

    fn get_bounding_box(&self) -> AabbBoundingBox {
        let bb1 = self.triangle1.get_bounding_box();
        let bb2 = self.triangle2.get_bounding_box();

        AabbBoundingBox {
            min_point: Vec3::new(
                bb1.min_point.x().min(bb2.min_point.x()),
                bb1.min_point.y().min(bb2.min_point.y()),
                bb1.min_point.z().min(bb2.min_point.z()),
            ),
            max_point: Vec3::new(
                bb1.max_point.x().max(bb2.max_point.x()),
                bb1.max_point.y().max(bb2.max_point.y()),
                bb1.max_point.z().max(bb2.max_point.z()),
            ),
        }
    }
}
