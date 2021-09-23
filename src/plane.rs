use crate::material::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::material::Material;
use crate::material::metal::Metal;
use crate::ray::Ray;
use crate::vec3::{cross, dot, Vec3};
use std::process::exit;
use std::sync::Arc;

// TODO(chesetti): THIS FILE COULD USE A CLEANUP!!!!
// TODO(chesetti): abstract out geometry from material.

pub struct Plane {
    point: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Send + Sync>,
}

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    normal: Vec3,
    material: Box<dyn Material + Send + Sync>,
}

pub struct Cube {
    pub center: Vec3,
    pub scale: f64,
    pub to: Vec3,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Cube {
    pub fn new(
        center: Vec3,
        scale: f64,
        to: Vec3,
        material: Box<dyn Material + Send + Sync>,
    ) -> Cube {
        Cube {
            center,
            scale,
            to,
            material,
        }
    }
}

impl Plane {
    pub fn xy_plane(z: f64, material: Box<dyn Material + Send + Sync>) -> Plane {
        Plane {
            point: Vec3::new(0.0, 0.0, z),
            normal: Vec3::new(0.0, 0.0, 1.0),
            material,
        }
    }

    pub fn zx_plane(y: f64, material: Box<dyn Material + Send + Sync>) -> Plane {
        Plane {
            point: Vec3::new(0.0, y, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material,
        }
    }

    pub fn yz_plane(x: f64, material: Box<dyn Material + Send + Sync>) -> Plane {
        Plane {
            point: Vec3::new(x, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            material,
        }
    }
}

impl Triangle {
    pub fn new(
        p1: Vec3,
        p2: Vec3,
        p3: Vec3,
        material: Box<dyn Material + Send + Sync>,
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

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let den = dot(ray.direction(), &self.normal);
        if den.abs() < 1e-6 {
            return None;
        }
        let num = dot(&(self.point - ray.origin()), &self.normal);
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
            material: Arc::new(&Box::new(&self.material)),
        })
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
        fn sameSide(p1: Vec3, p2: Vec3, a: Vec3, b: Vec3) -> bool {
            let cv1 = cross(&(b - a), &(p1 - a));
            let cv2 = cross(&(b - a), &(p2 - a));
            let d = dot(&cv1, &cv2);
            d >= 0.0
        }

        if sameSide(self.p1, hit_point, self.p2, self.p3)
            && sameSide(self.p2, hit_point, self.p3, self.p1)
            && sameSide(self.p3, hit_point, self.p1, self.p2)
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
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (self.center - self.to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let mut points = vec![
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
