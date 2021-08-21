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

use crate::camera::Camera;
use crate::camera::PixelRays;
use crate::color::Color;
use crate::hittable::{Hittable, HitRecord};
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use std::f64::consts::PI;

fn ray_color(spheres: &Vec<&Sphere>, ray: &Ray, depth: u32) -> Color {
    if depth <= 0 {
        Color::black();
    }
    let mut nearest_hit_record: Option<HitRecord> = None;
    let mut nearest_t = 0.0;

    for sphere in spheres.iter() {
        let maybe_hit_record = sphere.hit(&ray, 0.0001, f64::MAX);
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
        let nearest_hit_record = nearest_hit_record.unwrap();
        let scatter_result = nearest_hit_record.material.scatter(ray, &nearest_hit_record);
        match scatter_result {
            Some(scatter_result) => {
                return ray_color(spheres, &scatter_result.ray, depth - 1)
                    .attenuate(scatter_result.attenuation);
            }
            None => Color::black(),
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let background_param = 0.5 * (unit_direction.y() + 1.0);
        let blueish = Color::new(0.5, 0.7, 1.0);
        Color::lerp(Color::white(), blueish, background_param)
    }
}

fn main() {
    // Camera & Viewport
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;

    let camera = Camera::camera(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        16.0 / 9.0,
        img_width,
        img_height,
    );

    let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // Sphere in real world coordinates
    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Lambertian::new(Vec3::new(1.0, 0.5, 0.5))),
    };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    };
    let sphere3 = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal::new(Vec3::new(0.8, 0.8, 0.8))),
    };
    let sphere4 = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(Metal::new(Vec3::new(0.2, 0.7, 0.3))),
    };

    let world = vec![&sphere2, &sphere3, &sphere1, &sphere4];

    let samples_per_pixel: u32 = 100;
    let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

    for pixel_ray in pixel_rays {
        let mut sampled_colors: Vec<Color> = vec![];
        for ray in pixel_ray.rays {
            sampled_colors.push(ray_color(&world, &ray, 100));
        }

        let color = Color::average_color(sampled_colors.iter()).gamma_corrected();
        img_buf.put_pixel(pixel_ray.x, pixel_ray.y, color.image_pixel());
    }
    img_buf.save("gradient.png").unwrap();
}
