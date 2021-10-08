use crate::geometry::ray::Ray;
use crate::geometry::vec3::{cross, Vec3};
use crate::hittable::bounding_box::AabbBoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use crate::hittable::triangle::Triangle;
use crate::material::color::Color;
use crate::material::material::Material;
use crate::material::metal::Metal;
use std::sync::Arc;

pub struct Cube {
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    material: Arc<Box<dyn Material + Send + Sync>>,
    points: Vec<Vec3>,
}

impl Cube {
    pub fn new(
        center: Vec3,
        scale: f64,
        to: Vec3,
        material: Arc<Box<dyn Material + Send + Sync>>,
    ) -> Cube {
        // TODO(chesetti): Add rotation around (center - to)
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (center - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let points = vec![
            center + scale * (forward + right + up),
            center + scale * (forward + right - up),
            center + scale * (forward - right + up),
            center + scale * (forward - right - up),
            center + scale * ((-1.0 * forward) + right + up),
            center + scale * ((-1.0 * forward) + right - up),
            center + scale * ((-1.0 * forward) - right + up),
            center + scale * ((-1.0 * forward) - right - up),
        ];

        Cube {
            forward,
            right,
            up,
            points,
            material,
        }
    }

    pub fn newCuboid(
        center: Vec3,
        to: Vec3,
        width: f64,
        height: f64,
        depth: f64,
        material: Arc<Box<dyn Material + Send + Sync>>,
    ) -> Cube {
        // TODO(chesetti): Add rotation around (center - to)
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (center - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let forward = forward * (depth * 0.5);
        let right = right * (width * 0.5);
        let up = up * (height * 0.5);

        let points = vec![
            center + (forward + right + up),
            center + (forward + right - up),
            center + (forward - right + up),
            center + (forward - right - up),
            center + ((-1.0 * forward) + right + up),
            center + ((-1.0 * forward) + right - up),
            center + ((-1.0 * forward) - right + up),
            center + ((-1.0 * forward) - right - up),
        ];

        Cube {
            forward,
            right,
            up,
            points,
            material,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        fn triangle(points: &Vec<Vec3>, p1: usize, p2: usize, p3: usize) -> Triangle {
            // TODO(chesetti): decouple materials and objects.
            // TODO(chesetti): OR implement Cube as a Bounding Volume of Triangles.
            let dummy_material: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(Metal::new(Color::new(0.7, 0.7, 0.6), 0.0)));
            Triangle::new(points[p1 - 1], points[p2 - 1], points[p3 - 1], Arc::clone(&dummy_material))
        }

        let triangles = vec![
            // Face 1
            triangle(&self.points, 1, 2, 3),
            triangle(&self.points, 4, 2, 3),
            // Face 2
            triangle(&self.points, 5, 6, 7),
            triangle(&self.points, 8, 6, 7),
            // Face 3
            triangle(&self.points, 6, 2, 8),
            triangle(&self.points, 4, 2, 8),
            // Face 4
            triangle(&self.points, 1, 5, 3),
            triangle(&self.points, 7, 5, 3),
            // Face 5
            triangle(&self.points, 1, 5, 2),
            triangle(&self.points, 6, 5, 2),
            // Face 6
            triangle(&self.points, 3, 7, 4),
            triangle(&self.points, 8, 7, 4),
        ];

        let mut nearest_hit_record: Option<HitRecord> = None;
        let mut nearest_t = 0.0;

        for triangle in triangles.iter() {
            let maybe_hit_record = triangle.hit(&ray, t_min, t_max);
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

    fn get_bounding_box(&self) -> AabbBoundingBox {
        let mut min_x = self.points[0].x();
        let mut min_y = self.points[0].y();
        let mut min_z = self.points[0].z();

        let mut max_x = self.points[0].x();
        let mut max_y = self.points[0].y();
        let mut max_z = self.points[0].z();

        for p in &self.points {
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
