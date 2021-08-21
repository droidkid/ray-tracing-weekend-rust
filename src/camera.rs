use crate::ray::Ray;
use crate::vec3::{cross, Vec3};
use rand::Rng;

pub struct Camera {
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,

    // Viewport Stuff
    vertical_fov: f64,
    aspect_ratio: f64,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,

    // Viewport to Screen stuff
    raster_width: u32,
    raster_height: u32,
}

pub struct PixelRays {
    pub x: u32,
    pub y: u32,
    pub rays: Vec<Ray>,
}

impl Camera {
    pub fn camera(
        from: Vec3,
        to: Vec3,
        vup: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        raster_width: u32,
        raster_height: u32,
    ) -> Camera {
        // Reference: https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function
        let forward = (from - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        println!("{} {} {}", forward.x(), forward.y(), forward.z());
        println!("{} {} {}", right.x(), right.y(), right.z());
        println!("{} {} {}", up.x(), up.y(), up.z());

        let theta = vertical_fov.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width= aspect_ratio * viewport_height;

        Camera {
            position: from,
            forward,
            right,
            up,
            vertical_fov,
            aspect_ratio,
            focal_length: 1.0,
            viewport_width,
            viewport_height,
            raster_width,
            raster_height,
        }
    }

    // Function that returns a list of rays for each pixel in the raster
    pub fn get_rays(&self, samples_per_pixel: u32) -> Vec<PixelRays> {
        let viewport_center = self.position - self.forward;
        let viewport_lower_left = viewport_center
            + ((-0.5 * self.viewport_height) * self.up)
            + ((-0.5 * self.viewport_width) * self.right);

        let mut pixel_rays: Vec<PixelRays> = vec![];
        let mut rng = rand::thread_rng();

        for x in 0..self.raster_width {
            for y in 0..self.raster_height {
                let mut rays: Vec<Ray> = vec![];

                for _ in 0..samples_per_pixel {
                    let px: f64 = x as f64 + rng.gen::<f64>();
                    let py: f64 = y as f64 + rng.gen::<f64>();

                    let sx = self.viewport_width * (px / self.raster_width as f64);
                    let sy = self.viewport_height * (self.raster_height as f64 - py) / self.raster_height as f64;

                    let destination = viewport_lower_left + (sx * self.right) + (sy * self.up);
                    rays.push(Ray::from_to(self.position, destination));
                }
                pixel_rays.push(PixelRays { x, y, rays });
            }
        }
        pixel_rays
    }
}

#[test]
fn camera_look_at() {
    let from = Vec3::new(0.0, 0.0, 0.0);
    let to: Vec3 = Vec3::new(0.0, 0.0, -1.0);

    let cam = Camera::camera(from, to, 4.0, 2.0, 400, 225);

    assert_eq!(cam.forward, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(cam.right, Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(cam.up, Vec3::new(0.0, 1.0, 0.0));
}
