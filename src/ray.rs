use crate::vec3::Vec3;
use crate::vec3::dot;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

pub fn intersects_sphere(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc: Vec3 = ray.origin() - center;
    let a = dot(ray.direction(), ray.direction());
    let b = 2.0 * dot(&oc, ray.direction());
    let c = dot(&oc, &oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        Some((-b - discriminant.sqrt()) * 0.5 / a)
    } else {
        None
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn new_from_origin(direction: Vec3) -> Ray {
        Ray {
            origin: Vec3::origin(),
            direction: direction,
        }
    }

    pub fn origin(&self) -> &Vec3 {
        return &self.origin;
    }

    pub fn direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin() + (self.direction() * t)
    }
}

#[test]
fn initialize_ray() {
    let origin = Vec3::new(0.0, 1.0, 0.0);
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(ray.direction, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn initialize_ray_at_origin() {
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray::new_from_origin(direction);

    assert_eq!(ray.origin, Vec3::new(0.0, 0.0, 0.0));
    assert_eq!(ray.direction, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn project_ray() {
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray::new(origin, direction);

    let midpoint = ray.at(0.5);
    assert_eq!(midpoint, Vec3::new(0.5, 0.5, 0.5));

    let backwards = ray.at(-0.7);
    assert_eq!(backwards, Vec3::new(-0.7, -0.7, -0.7));
}
