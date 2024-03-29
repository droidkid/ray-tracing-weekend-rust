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
use crate::hittable::triangle::Triangle;
use crate::material::checkered_texture::CheckeredTexture;
use crate::material::dielectric::Dielectric;
use crate::material::image_texture::ImageTexture;
use crate::material::lambertian::Lambertian;
use crate::material::material::Material;
use crate::material::metal::Metal;
use crate::world::world::World;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::sync::atomic::Ordering;

mod geometry;
mod hittable;
mod material;
mod world;

fn load_obj(file: &str) -> Vec<Box<dyn Hittable + Send + Sync>> {
    let contents = fs::read_to_string(file).expect("Error Reading filename");
    let lines = contents.split("\n");
    let mut vertices: Vec<Vec3> = vec![];

    for line in lines.into_iter() {
        if line.starts_with("v ") {
            let points = line.split(" ").collect::<Vec<&str>>();

            let x = points[1].trim().parse::<f64>().unwrap();
            let y = points[2].trim().parse::<f64>().unwrap();
            let z = points[3].trim().parse::<f64>().unwrap();

            vertices.push(Vec3::new(x, y, z));
        }
    }
    let contents = fs::read_to_string(file).expect("Error Reading filename");
    let lines = contents.split("\n");

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = vec![];
    let grey: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        Lambertian::new_from_color(Color::new(0.5, 0.5, 0.5)),
    ));

    for line in lines.into_iter() {
        if line.starts_with("f ") {
            let points = line.split(" ").collect::<Vec<&str>>();
            let mut v: Vec<usize> = vec![];
            for i in 1..4 {
                let point_details = points[i].split("/").collect::<Vec<&str>>();
                v.push(point_details[0].trim().parse::<usize>().unwrap());
            }

            objects.push(Box::new(Triangle::new(
                vertices[v[0] - 1],
                vertices[v[1] - 1],
                vertices[v[2] - 1],
                Arc::clone(&grey),
            )));
        }
    }

    objects
}

fn cubes_and_spheres_scene() {
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

    let grey: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        Lambertian::new_from_color(Color::new(0.5, 0.5, 0.5)),
    ));
    let white: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Metal::new(Color::white(), 0.1)));
    let earth_texture: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        Lambertian::new_from_texture(Box::new(ImageTexture::new("earthmap.jpg"))),
    ));
    let mars_texture: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        Lambertian::new_from_texture(Box::new(ImageTexture::new("mars.jpg"))),
    ));

    let ground = Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Arc::clone(&grey),
    };

    let earth = Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Arc::clone(&earth_texture),
    };

    let mars = Sphere {
        center: Vec3::new(4.0, 1.0, -3.5),
        radius: 1.0,
        material: Arc::clone(&mars_texture),
    };

    let cube1 = Cube::new(
        Vec3::new(-8.0, 4.0, 0.0),
        3.0,
        Vec3::new(1.0, 1.0, 0.5),
        Arc::clone(&white),
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
            let material: Arc<Box<dyn Material + Send + Sync>>;
            if choose_mat < 0.8 {
                material = Arc::new((Box::new(Metal::new(Color::random(), 0.1))));
            } else if choose_mat < 0.95 {
                material = Arc::new(Box::new(Lambertian::new_from_color(Color::random())));
            } else {
                material = Arc::new(Box::new(Dielectric::new(1.5)));
            }

            let choose_cube = rng.gen::<f64>();
            let object: Box<dyn Hittable + Send + Sync>;

            if choose_cube < 0.1 {
                object = Box::new(Cube::new(
                    center,
                    0.3,
                    Vec3::new(1.0, 0.5, 0.0),
                    Arc::clone(&material),
                ));
            } else {
                object = Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Arc::clone(&material),
                });
            }

            objects.push(object);
        }
    }

    let num_objects = objects.len();
    let world = World::new(objects);

    let now = Instant::now();
    world::world::render(
        world,
        "cubes_spheres_render.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::white(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!("Num Objects: {}", num_objects);
    println!(
        "Intersection Count: {}",
        hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed)
    );
}

fn cornell_box_scene() {
    let white: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Lambertian::new_from_color(Color::white())));

    let right_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(0.0, 0.0, 0.0),
        Color::new(0.65, 0.05, 0.05),
    );

    let left_wall = Quad::new_lambertian(
        Vec3::new(555.0, 555.0, 0.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Color::new(0.12, 0.45, 0.15),
    );

    let back_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(0.0, 0.0, 555.0),
        Color::white(),
    );

    let light = Quad::new_diffuse_light(
        Vec3::new(113.0, 554.0, 127.0),
        Vec3::new(113.0, 554.0, 432.0),
        Vec3::new(443.0, 554.0, 432.0),
        Vec3::new(443.0, 554.0, 127.0),
        Color::new(1.0, 1.0, 1.0),
    );

    let top_wall = Quad::new_lambertian(
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(555.0, 555.0, 0.0),
        Color::white(),
    );

    let bottom_wall = Quad::new_lambertian(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Color::white(),
    );

    let cube1 = Cube::newCuboid(
        Vec3::new(138.0, 75.0, 130.0),
        Vec3::new(200.0, 75.0, 300.0),
        100.0,
        150.0,
        100.0,
        Arc::clone(&white),
    );

    let cube2 = Cube::newCuboid(
        Vec3::new(400.0, 150.0, 330.0),
        Vec3::new(100.0, 150.0, 300.0),
        100.0,
        300.0,
        100.0,
        Arc::clone(&white),
    );

    let objects: Vec<Box<dyn Hittable + Sync + Send>> = vec![
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
        "cornell_scene.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::black(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!(
        "Intersection Count: {}",
        hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed)
    );
}

fn die_box_scene() {
    let back_wall_material: Arc<Box<dyn Material + Send + Sync>> =
        Arc::new(Box::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.01)));

    let back_wall = Quad::new(
        Vec3::new(-5050.0, 10555.0, 6055.0),
        Vec3::new(55055.0, 10555.0, 6055.0),
        Vec3::new(55055.0, -1000.0, 0.0),
        Vec3::new(-5050.0, -1000.0, 0.0),
        &back_wall_material,
    );

    let bottom_wall = Quad::new_lambertian(
        Vec3::new(-5000.0, 0.0, 0.0),
        Vec3::new(-5000.0, 0.0, 555.0),
        Vec3::new(5555.0, 0.0, 555.0),
        Vec3::new(5555.0, 0.0, 0.0),
        Color::new(0.65, 0.12, 0.12),
    );

    let cube1 = Cube::new_mapped_cube(
        "uv-map.png",
        Vec3::new(138.0, 225.0, 230.0),
        Vec3::new(200.0, 105.0, 300.0),
        200.0,
        250.0,
        250.0,
    );

    let cube2 = Cube::new_mapped_cube(
        "uv-map-2.png",
        Vec3::new(500.0, 350.0, 330.0),
        Vec3::new(200.0, 500.0, 200.0),
        150.0,
        270.0,
        200.0,
    );

    let objects: Vec<Box<dyn Hittable + Sync + Send>> = vec![
        Box::new(bottom_wall),
        Box::new(back_wall),
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
        "die_scene.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::white(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!(
        "Intersection Count: {}",
        hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed)
    );
}

fn object_loader_scene(obj_file: &str) {
    // Camera & Viewport
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 300;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let recursive_depth: u32 = 100;
    let num_threads = 16;

    let camera = Camera::camera(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        5.0,
        50.0,
        aspect_ratio,
        img_width,
        img_height,
    );

    let grey: Arc<Box<dyn Material + Send + Sync>> = Arc::new(Box::new(
        Lambertian::new_from_color(Color::new(0.5, 0.5, 0.5)),
    ));

    let ground = Sphere {
        center: Vec3::new(0.0, -7.0, 0.0),
        radius: 6.0,
        material: Arc::clone(&grey),
    };

    let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = load_obj(obj_file);
    objects.push(Box::new(ground));

    let num_objects = objects.len();
    println!("Num Objects: {}", num_objects);

    let world = World::new(objects);
    println!("Starting rendering!: {}", num_objects);

    let now = Instant::now();
    world::world::render(
        world,
        "face.png",
        &camera,
        samples_per_pixel,
        recursive_depth,
        num_threads,
        Color::white(),
    );
    let elapsed = now.elapsed();
    println!("Wrote render.png in {} seconds", elapsed.as_secs());
    println!(
        "Intersection Count: {}",
        hittable::bounding_box_tree::COUNTER.fetch_add(0, Ordering::Relaxed)
    );
}

fn main() {
    // TODO(chesetti): Add an argument that lets you choose what scene to render.
    object_loader_scene("head.obj");
}
