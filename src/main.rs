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
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use rand::Rng;

fn ray_color(spheres: &Vec<Sphere>, ray: &Ray, depth: u32) -> Color {
    if depth <= 0 {
        return Color::black();
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
        let scatter_result = nearest_hit_record
            .material
            .scatter(ray, &nearest_hit_record);
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

    // Sphere in real world coordinates
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

    let mut world = vec![ground, sphere2, sphere1, sphere3];
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
            world.push(sphere);
        }
    }

    let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

    let mut img_buf = image::ImageBuffer::new(img_width, img_height);
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
