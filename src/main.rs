extern crate image;
#[macro_use]
extern crate impl_ops;

mod camera;
mod hittable;
mod ray;
mod vec3;
mod color;

use crate::camera::PixelRays;
use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::hittable::Sphere;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;

fn random_in_unit_sphere()  -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}

fn ray_color(spheres: &Vec<&Sphere>, ray: &Ray, depth: u32) -> Color {
    if depth <= 0 {
        Color::black();
    }

    for sphere in spheres.iter() {
        let sphere_hit_record = sphere.hit(&ray, 0.0001, f64::MAX);
        if sphere_hit_record.is_none() {
            continue;
        }
        let hit_record = sphere_hit_record.unwrap();

        let target = hit_record.hit_point + hit_record.normal + random_in_unit_sphere();
        let next_ray = Ray::from_to(hit_record.hit_point, target);
        return ray_color(spheres, &next_ray, depth-1).intensity(0.5);
    };

     let unit_direction = ray.direction().normalize();
     let background_param = 0.5 * (unit_direction.y() + 1.0);
     let blueish = Color::new(0.5, 0.7, 1.0);
     Color::lerp(Color::white(), blueish, background_param)
}

fn main() {
    // Camera & Viewport
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;

    let camera = Camera::camera(
        Vec3::origin(),
        Vec3::new(0.0, 0.0, -1.0),
        viewport_width,
        viewport_height,
        img_width,
        img_height,
    );

    let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // Sphere in real world coordinates
    let sphere1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let sphere2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let world = vec![&sphere1, &sphere2];

    let samples_per_pixel: u32 = 100;
    let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

    for pixel_ray in pixel_rays {
        let mut sampled_colors: Vec<Color> = vec![];
        for ray in pixel_ray.rays {
            sampled_colors.push(ray_color(&world, &ray, 100));
        }

        let color = Color::average_color(sampled_colors.iter()).gamma_corrected();
        img_buf.put_pixel(
            pixel_ray.x,
            pixel_ray.y,
            color.image_pixel()
        );
    }
    img_buf.save("gradient.png").unwrap();
}
