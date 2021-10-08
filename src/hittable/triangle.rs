use crate::geometry::ray::Ray;
use crate::geometry::vec3::{cross, dot, Vec3};
use crate::hittable::bounding_box::AabbBoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use crate::material::material::Material;
use std::sync::Arc;

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    normal: Vec3,
    material: Arc<Box<dyn Material + Send + Sync>>,
}

impl Triangle {
    pub fn new(
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        material: Arc<Box<dyn Material + Send + Sync>>,
    ) -> Triangle {
        let v1 = p2 - p1;
        let v2 = p3 - p1;

        // We don't care about front side or back side for now.
        let normal = cross(&v1, &v2).normalize();

        Triangle {
            p1,
            p2,
            p3,
            normal,
            material,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let den = dot(ray.direction(), &self.normal);
        if den.abs() < 1e-6 {
            return None;
        }
        let num = dot(&(self.p1 - ray.origin()), &self.normal);
        let t = num / den;

        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);

        // TODO(chesetti): This could be faster, but this method is the simplest. Let's optimize later.
        // Taken from https://blackpawn.com/texts/pointinpoly/
        fn same_side(p1: Vec3, p2: Vec3, a: Vec3, b: Vec3) -> bool {
            let cv1 = cross(&(b - a), &(p1 - a));
            let cv2 = cross(&(b - a), &(p2 - a));
            let d = dot(&cv1, &cv2);
            d >= 0.0
        }

        if same_side(self.p1, hit_point, self.p2, self.p3)
            && same_side(self.p2, hit_point, self.p3, self.p1)
            && same_side(self.p3, hit_point, self.p1, self.p2)
        {
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
                material: Arc::new(&Box::new(&self.material)),
            })
        } else {
            None
        }
    }

    fn get_bounding_box(&self) -> AabbBoundingBox {
        let points = vec![
            self.p1 + self.normal,
            self.p1 - self.normal,
            self.p2 + self.normal,
            self.p2 - self.normal,
            self.p3 + self.normal,
            self.p3 - self.normal,
        ];

        let mut min_x = points[0].x();
        let mut min_y = points[0].y();
        let mut min_z = points[0].z();

        let mut max_x = points[0].x();
        let mut max_y = points[0].y();
        let mut max_z = points[0].z();

        for p in points {
            min_x = min_x.min(p.x());
            min_y = min_y.min(p.y());
            min_z = min_z.min(p.z());

            max_x = max_x.max(p.x());
            max_y = max_y.max(p.y());
            max_z = max_z.max(p.z());
        }

        AabbBoundingBox {
            min_point: Vec3::new(min_x, min_y, min_z),
            max_point: Vec3::new(max_x, max_y, max_z),
        }
    }
}
