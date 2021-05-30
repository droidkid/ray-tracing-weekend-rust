extern crate image;
#[macro_use]
extern crate impl_ops;

mod hittable;
mod ray;
mod vec3;

use crate::hittable::HitRecord;
use hittable::Hittable;
use hittable::Sphere;
use ray::Ray;
use vec3::Vec3;

fn main() {
    // Colors
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blueish = Vec3::new(0.5, 0.7, 1.0);

    let aspect_ratio = 16.0 / 9.0;

    // Image.
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // Camera.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

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

    // Eye: (0,0,0).
    // Viewport (0, 0, -focal_length).
    let center_of_viewport = Vec3::new(0.0, 0.0, -1.0 * focal_length);
    let upper_left_corner =
        center_of_viewport + Vec3::new(-0.5 * viewport_width, 0.5 * viewport_height, 0.0);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        // x, y are pixel coordinates.
        // Let's convert them to viewport coordinates.
        let delta = Vec3::new(
            (x as f64) / (img_width as f64) * viewport_width,
            // The Y coordinate in viewport and image are reverse.
            // Y goes positive down in image.
            // Y goes  negative down in viewport.
            -1.0 * (y as f64) / (img_height as f64) * viewport_height,
            0.0,
        );
        let viewport_coordinate = upper_left_corner + delta;
        let ray = Ray::new_from_origin(viewport_coordinate);

        let unit_direction = ray.direction().unit_vector();
        let background_param = 0.5 * (unit_direction.y() + 1.0);

        let mut hit_record: Option<HitRecord> = None;

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

        let r = (color.x() * 256.0) as u8;
        let g = (color.y() * 256.0) as u8;
        let b = (color.z() * 256.0) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    img_buf.save("gradient.png").unwrap();
}
