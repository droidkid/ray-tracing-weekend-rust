extern crate image;
#[macro_use]
extern crate impl_ops;

mod camera;
mod hittable;
mod ray;
mod vec3;

use crate::camera::PixelRays;
use crate::hittable::HitRecord;
use camera::Camera;
use hittable::Hittable;
use hittable::Sphere;
use vec3::Vec3;

fn main() {
    // Colors
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blueish = Vec3::new(0.5, 0.7, 1.0);

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

    let samples_per_pixel: u32 = 50;
    let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

    for pixel_ray in pixel_rays {
        let mut r: f64 = 0.0;
        let mut g: f64 = 0.0;
        let mut b: f64 = 0.0;

        for ray in pixel_ray.rays {
            let mut hit_record: Option<HitRecord> = None;

            let unit_direction = ray.direction().normalize();
            let background_param = 0.5 * (unit_direction.y() + 1.0);

            for object in world.iter() {
                let object_hit_record = object.hit(&ray, 0.0, f64::MAX);

                if object_hit_record.is_none() {
                    continue;
                }

                let t = match &object_hit_record {
                    Some(object_hit_record) => Some(object_hit_record.t),
                    None => None, // Should not reach here.
                }
                .unwrap();

                hit_record = match hit_record {
                    Some(hit_record) => {
                        if hit_record.t > t {
                            object_hit_record
                        } else {
                            Some(hit_record)
                        }
                    }
                    None => object_hit_record,
                };
            }

            let color = match hit_record {
                Some(hit_point_record) => {
                    let normal = hit_point_record.normal;
                    Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5
                }
                None => white * (1.0 - background_param) + blueish * background_param,
            };

            r = r + color.x();
            g = g + color.y();
            b = b + color.z();
        }

        r = r * 256.0 / (samples_per_pixel as f64);
        g = g * 256.0 / (samples_per_pixel as f64);
        b = b * 256.0 / (samples_per_pixel as f64);

        img_buf.put_pixel(
            pixel_ray.x,
            pixel_ray.y,
            image::Rgb([r as u8, g as u8, b as u8]),
        );
    }
    img_buf.save("gradient.png").unwrap();
}
