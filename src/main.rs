extern crate image;
#[macro_use]
extern crate impl_ops;

use std::sync::Arc;
use std::time::Instant;

use rand::Rng;

use geometry::vec3::Vec3;
use hittable::quad::Quad;
use hittable::sphere::Sphere;
use material::color::Color;
use world::camera::Camera;

use crate::hittable::bounding_box_tree::BoundingBoxTree;
use crate::hittable::cube::Cube;
use crate::hittable::hittable::Hittable;
use crate::material::checkered_texture::CheckeredTexture;
use crate::material::dielectric::Dielectric;
use crate::material::image_texture::ImageTexture;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::world::world::World;
use std::path::Path;
use std::fmt::Debug;
use std::sync::atomic::Ordering;

mod geometry;
mod hittable;
mod material;
mod world;

fn cubes_and_spheres_scene()  {
    // Camera & Viewport
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 300;
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
    let earth = Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(Lambertian::new_from_texture(Box::new(ImageTexture::new(
            "earthmap.jpg",
        )))),
    };

    let mars = Sphere {
        center: Vec3::new(4.0, 1.0, -3.5),
        radius: 1.0,
        material: Box::new(Lambertian::new_from_texture(Box::new(ImageTexture::new(
            "mars.jpg",
        )))),
    };

    let cube1 = Cube::new(
        Vec3::new(-8.0, 4.0, 0.0),
        3.0,
        Vec3::new(1.0, 1.0, 0.5),
        (Box::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0))),
    );

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = vec![
        Box::new(ground),
        Box::new(cube1),
        Box::new(earth),
        Box::new(mars),
    ];

    let mut rng = rand::thread_rng();
    for a in -12..12 {
        for b in -12..12 {
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
                    cube = Cube::new(
                        center,
                        0.3,
                        Vec3::new(1.0, 0.5, 0.0),
                        (Box::new(Metal::new(Color::random(), 0.1))),
                    );
                } else if choose_mat < 0.95 {
                    cube = Cube::new(
                        center,
                        0.3,
                        Vec3::new(1.0, 0.5, 0.0),
                        Box::new(Metal::new(Color::random(), 0.0)),
                    )
                } else {
                    cube = Cube::new(
                        center,
                        0.3,
                        Vec3::new(1.0, 0.5, 0.0),
                        Box::new(Dielectric::new(1.5)),
                    )
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
    let num_objects = objects.len();
    let world = World::new(objects);

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
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!("Num Objects: {}", num_objects);
    println!("Intersection Count: {}", hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed));
}

fn cornell_box_scene()  {

    let right_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(0.0, 0.0, 0.0),
        Color::new(0.65, 0.05, 0.05)
    );

    let left_wall = Quad::new_lambertian(
        Vec3::new(555.0, 555.0, 0.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Color::new(0.12, 0.45, 0.15)
    );

    let back_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::white()
    );

    let light = Quad::new_diffuse_light(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(113.0, 554.0, 432.0),
        Vec3::new(443.0, 554.0, 432.0),
        Vec3::new(443.0, 554.0, 127.0),
        Color::new(1.0, 1.0, 1.0)
    );

    let top_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 0.0),
        Color::white()
    );

    let bottom_wall = Quad::new_lambertian(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Color::white()
    );
    
    let cube1 = Cube::newCuboid(Vec3::new(138.0, 75.0, 130.0),
                                Vec3::new(200.0, 75.0, 300.0),
                          100.0,
                          150.0,
                          100.0,
                          Box::new(Lambertian::new_from_color(Color::white())));

    let cube2 = Cube::newCuboid(Vec3::new(400.0, 150.0, 330.0),
                                Vec3::new(100.0, 150.0, 300.0),
                                100.0,
                                300.0,
                                100.0,
                                Box::new(Lambertian::new_from_color(Color::white())));

    let objects : Vec<Box<dyn Hittable + Sync + Send>>= vec![
      Box::new(right_wall),
      Box::new(left_wall),
      Box::new(back_wall),
      Box::new(top_wall),
      Box::new(bottom_wall),
      Box::new(light),
      Box::new(cube1),
      Box::new(cube2),
    ];

    // Camera & Viewport
    let aspect_ratio = 1.0;
    let img_width = 600;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 200;
    let recursive_depth: u32 = 100;
    let num_threads = 16;

    let camera = Camera::camera(
        Vec3::new(278.0, 278.0, -800.0),
        Vec3::new(278.0, 278.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        800.0,
        40.0,
        aspect_ratio,
        img_width,
        img_height,
    );


    let world = World::new(objects);

    let now = Instant::now();
    world::world::render(
        world,
        "render.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::black(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!("Intersection Count: {}", hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed));
}

fn main() {
    cornell_box_scene();
}