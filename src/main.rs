extern crate image;
#[macro_use]
extern crate impl_ops;

use std::sync::Arc;
use std::time::Instant;

use rand::Rng;

use geometry::vec3::Vec3;
use hittable::plane::Plane;
use hittable::sphere::Sphere;
use material::color::Color;
use world::camera::Camera;

use crate::hittable::cube::Cube;
use crate::hittable::hittable::Hittable;
use crate::material::checkered_texture::CheckeredTexture;
use crate::material::dielectric::Dielectric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::world::world::World;

mod geometry;
mod material;
mod world;
mod hittable;

fn main() {
    // Camera & Viewport
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 1200;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let recursive_depth: u32 = 100;
    let num_threads = 16;

    let camera = Camera::camera(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        10.0,
        50.0,
        aspect_ratio,
        img_width,
        img_height,
    );

    let ground = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: (Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))),
    };
    let sphere1 = Sphere {
        material: Box::new(Dielectric::new(1.5)),
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
    };
    let sphere2 = Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: (Box::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0))),
    };

    let cube1 = Cube {
        center: Vec3::new(4.0, 1.0, 0.0),
        to: Vec3::new(1.0, 1.0, 0.0),
        scale: 0.75,
        material: (Box::new(Metal::new(Color::new(0.5, 0.5, 0.2), 0.2))),
    };

    let checkered_texture = Box::new(CheckeredTexture::new(Color::random(), Color::random(), 1.0));
    let plane = Plane::xy_plane(
        -10.0,
        Box::new(Lambertian::new_from_texture(checkered_texture)),
    );

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> =
        vec![Box::new(ground), Box::new(cube1), Box::new(plane)];

    let mut rng = rand::thread_rng();
    for a in -6..6 {
        for b in -6..6 {
            let center = Vec3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.5,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).len() < 3.5 {
                continue;
            }

            let choose_mat = rng.gen::<f64>();
            let choose_cube = rng.gen::<f64>();

            if choose_cube < 0.5 {
                let cube;
                if choose_mat < 0.8 {
                    cube = Cube {
                        center,
                        to: Vec3::new(1.0, 0.5, 0.0),
                        scale: 0.3,
                        material: (Box::new(Metal::new(Color::random(), 0.1))),
                    };
                } else if choose_mat < 0.95 {
                    cube = Cube {
                        center,
                        to: Vec3::new(1.0, 0.5, 0.0),
                        scale: 0.3,
                        material: Box::new(Metal::new(Color::random(), 0.0)),
                    }
                } else {
                    cube = Cube {
                        center,
                        to: Vec3::new(1.0, 0.5, 0.0),
                        scale: 0.3,
                        material: Box::new(Dielectric::new(1.5)),
                    }
                }
                objects.push(Box::new(cube));
            } else if choose_cube < 0.8 {
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
                        material: Box::new(Metal::new(Color::random(), 0.0)),
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
    }

    let world = World::new(Arc::new(objects));

    let now = Instant::now();
    world::world::render(
        world,
        "render.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::white(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs())
}
