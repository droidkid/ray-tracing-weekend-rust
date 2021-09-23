use crate::camera::{Camera, PixelRays};
use crate::hittable::{HitRecord, Hittable};

use crate::geometry::ray::Ray;
use crate::material::color::Color;
use image::ImageBuffer;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct World {
    objects: Arc<Vec<Box<dyn Hittable + Send + Sync>>>,
}

impl World {
    pub fn new(objects: Arc<Vec<Box<dyn Hittable + Send + Sync>>>) -> World {
        World { objects }
    }
}

pub fn render(
    world: World,
    filename: &str,
    camera: &Camera,
    samples_per_pixel: u32,
    recursive_depth: u32,
    num_threads: u32,
    background_color: Color,
) {
    let img_buf = ImageBuffer::new(camera.raster_width, camera.raster_height);
    let pixel_rays: Vec<PixelRays> = camera.get_rays(samples_per_pixel);

    let img_buf_mutex = Arc::new(Mutex::new(img_buf));
    let pixel_rays_mutex = Arc::new(Mutex::new(pixel_rays));
    let objects_arc = Arc::clone(&world.objects);

    let mut handlers = vec![];
    for _ in 0..num_threads {
        let img_buf_thread_copy = Arc::clone(&img_buf_mutex);
        let pixel_rays_thread_copy = Arc::clone(&pixel_rays_mutex);
        let objects_copy = Arc::clone(&objects_arc);

        let handle = thread::spawn(move || loop {
            let pixel_rays;
            {
                let mut pixel_rays_queue = pixel_rays_thread_copy.lock().unwrap();
                if pixel_rays_queue.is_empty() {
                    break;
                }
                pixel_rays = pixel_rays_queue.pop().unwrap();
            }
            let color = get_pixel_color(
                &*objects_copy,
                &pixel_rays,
                recursive_depth,
                background_color,
            );
            {
                let mut img_buf = img_buf_thread_copy.lock().unwrap();
                img_buf.put_pixel(pixel_rays.x, pixel_rays.y, color.image_pixel());
            }
        });

        img_buf_mutex
            .lock()
            .unwrap()
            .put_pixel(0, 0, Color::white().image_pixel());
        handlers.push(handle);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    img_buf_mutex.lock().unwrap().save(filename).unwrap();
}

fn get_pixel_color(
    objects: &Vec<Box<dyn Hittable + Send + Sync>>,
    pixel_ray: &PixelRays,
    recursive_depth: u32,
    background: Color,
) -> Color {
    let mut sampled_colors: Vec<Color> = vec![];
    for ray in pixel_ray.rays.iter() {
        sampled_colors.push(ray_color(objects, &ray, recursive_depth, background));
    }
    Color::average_color(sampled_colors.iter()).gamma_corrected()
}

fn ray_color(
    objects: &Vec<Box<dyn Hittable + Send + Sync>>,
    ray: &Ray,
    depth: u32,
    background: Color,
) -> Color {
    if depth <= 0 {
        return Color::white();
    }
    let mut nearest_hit_record: Option<HitRecord> = None;
    let mut nearest_t = 0.0;

    for object in objects.iter() {
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

        return if scatter_result.scattered_ray.is_some() {
            scatter_result.emitted
                + ray_color(
                    objects,
                    &scatter_result.scattered_ray.unwrap(),
                    depth - 1,
                    background,
                )
                .attenuate(scatter_result.attenuation)
        } else {
            scatter_result.emitted
        };
    } else {
        background
    }
}
