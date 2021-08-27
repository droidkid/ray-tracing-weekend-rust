extern crate image;
#[macro_use]
extern crate impl_ops;

mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod world;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;
use crate::world::World;


fn main() {
    // Camera & Viewport
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 1200;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;

    let camera = Camera::camera(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        10.0,
        20.0,
        aspect_ratio,
        img_width,
        img_height,
    );

    let ground = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    };

    let sphere1 = Sphere {
        material: Box::new(Dielectric::new(1.5)),
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
    };
    let sphere2 = Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian::new(Vec3::new(1.0, 0.5, 0.5))),
    };
    let sphere3 = Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Metal::new(Vec3::new(0.7, 0.7, 0.6), 0.0)),
    };

    let mut objects: Vec<Box<dyn Hittable>> = vec![Box::new(ground), Box::new(sphere2), Box::new(sphere1), Box::new(sphere3)];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() < 0.9 {
                continue;
            }

            let choose_mat = rng.gen::<f64>();
            let sphere;
            if choose_mat < 0.8 {
                sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Lambertian::new(Vec3::random(0.0, 1.0))),
                }
            } else if choose_mat < 0.95 {
                sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Metal::new(Vec3::random(0.0, 1.0), 0.0)),
                }
            } else {
                sphere = Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Dielectric::new(1.5)),
                }
            }
            objects.push(Box::new(sphere));
        }
    }

    let world = World::new(objects);
    world.render("render.png", &camera, samples_per_pixel);
}
