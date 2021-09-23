use crate::geometry::vec3::{Vec3, cross};
use crate::material::material::Material;
use crate::hittable::hittable::{Hittable, HitRecord};
use crate::geometry::ray::Ray;
use crate::hittable::triangle::Triangle;
use crate::material::metal::Metal;
use crate::material::color::Color;
use std::sync::Arc;

pub struct Cube {
    pub center: Vec3,
    pub scale: f64,
    pub to: Vec3,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (self.center - self.to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let points = vec![
            self.center + self.scale * (forward + right + up),
            self.center + self.scale * (forward + right - up),
            self.center + self.scale * (forward - right + up),
            self.center + self.scale * (forward - right - up),
            self.center + self.scale * ((-1.0 * forward) + right + up),
            self.center + self.scale * ((-1.0 * forward) + right - up),
            self.center + self.scale * ((-1.0 * forward) - right + up),
            self.center + self.scale * ((-1.0 * forward) - right - up),
        ];

        fn triangle(points: &Vec<Vec3>, p1: usize, p2: usize, p3: usize) -> Triangle {
            // TODO(chesetti): decouple materials and objects.
            // TODO(chesetti): OR implement Cube as a Bounding Volume of Triangles.
            let material = Box::new(Metal::new(Color::new(0.7, 0.7, 0.6), 0.0));
            Triangle::new(points[p1 - 1], points[p2 - 1], points[p3 - 1], material)
        }

        let triangles = vec![
            // Face 1
            triangle(&points, 1, 2, 3),
            triangle(&points, 4, 2, 3),
            // Face 2
            triangle(&points, 5, 6, 7),
            triangle(&points, 8, 6, 7),
            // Face 3
            triangle(&points, 6, 2, 8),
            triangle(&points, 4, 2, 8),
            // Face 4
            triangle(&points, 1, 5, 3),
            triangle(&points, 7, 5, 3),
            // Face 5
            triangle(&points, 1, 5, 2),
            triangle(&points, 6, 5, 2),
            // Face 6
            triangle(&points, 3, 7, 4),
            triangle(&points, 8, 7, 4),
        ];

        let mut nearest_hit_record: Option<HitRecord> = None;
        let mut nearest_t = 0.0;

        for triangle in triangles.iter() {
            let maybe_hit_record = triangle.hit(&ray, 0.0001, f64::MAX);
            if maybe_hit_record.is_none() {
                continue;
            }

            let hit_record = maybe_hit_record.unwrap();
            if nearest_hit_record.is_none() || hit_record.t < nearest_t {
                nearest_t = hit_record.t;
                nearest_hit_record = Some(hit_record);
            }
        }

        if nearest_hit_record.is_some() {
            let nhr = nearest_hit_record.unwrap();
            return Some(HitRecord {
                hit_point: nhr.hit_point,
                normal: nhr.normal,
                front_face: nhr.front_face,
                t: nhr.t,
                u: 0.0, // TODO(): implement
                v: 0.0, // TODO(): implement!
                material: Arc::new(&Box::new(&self.material)),
            });
        }

        None
    }
}
