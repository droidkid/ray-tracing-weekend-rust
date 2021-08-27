use crate::camera::{Camera, PixelRays};
use crate::hittable::{Hittable, HitRecord};

use image::ImageBuffer;
use crate::color::Color;
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> World {
        World {
            objects
        }
    }

    pub fn render(&self, filename: &str, camera: &Camera, samples_per_pixel: u32) {
        let mut img_buf = ImageBuffer::new(camera.raster_width, camera.raster_height);
        let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

        for pixel_ray in pixel_rays {
            let mut sampled_colors: Vec<Color> = vec![];
            for ray in pixel_ray.rays {
                sampled_colors.push(self.ray_color(&ray, 100));
            }

            let color = Color::average_color(sampled_colors.iter()).gamma_corrected();
            img_buf.put_pixel(pixel_ray.x, pixel_ray.y, color.image_pixel());
        }
        img_buf.save(filename).unwrap();
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        let mut nearest_hit_record: Option<HitRecord> = None;
        let mut nearest_t = 0.0;

        for object in self.objects.iter() {
            let maybe_hit_record = object.hit(&ray, 0.0001, f64::MAX);
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
                    return self.ray_color(&scatter_result.ray, depth - 1)
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
}

