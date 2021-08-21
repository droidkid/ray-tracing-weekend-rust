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

    // Lens Stuff
    aperture: f64,
    focus_dist: f64,

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
        aperture: f64,
        focus_dist: f64,
        vertical_fov: f64,
        aspect_ratio: f64,
        raster_width: u32,
        raster_height: u32,
    ) -> Camera {
        // Reference: https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function
        let forward = (from - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

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
            aperture,
            focus_dist,
            focal_length: 1.0,
            viewport_width,
            viewport_height,
            raster_width,
            raster_height,
        }
    }

    // Function that returns a list of rays for each pixel in the raster
    pub fn get_rays(&self, samples_per_pixel: u32) -> Vec<PixelRays> {
        let viewport_center = self.position - self.focus_dist * self.forward;
        let viewport_lower_left = viewport_center
            + ((-0.5 * self.viewport_height * self.focus_dist) * self.up)
            + ((-0.5 * self.viewport_width * self.focus_dist) * self.right);

        let mut pixel_rays: Vec<PixelRays> = vec![];
        let mut rng = rand::thread_rng();

        for x in 0..self.raster_width {
            for y in 0..self.raster_height {
                let mut rays: Vec<Ray> = vec![];

                for _ in 0..samples_per_pixel {
                    let px: f64 = x as f64 + rng.gen::<f64>();
                    let py: f64 = y as f64 + rng.gen::<f64>();

                    let sx = self.focus_dist * self.viewport_width * (px / self.raster_width as f64);
                    let sy = self.focus_dist * self.viewport_height * (self.raster_height as f64 - py) / self.raster_height as f64;
                    let destination = viewport_lower_left + (sx * self.right) + (sy * self.up);

                    let rd = (self.aperture * 0.5) * random_in_unit_disk();
                    let offset = self.right * rd.x() + self.up * rd.y();
                    rays.push(Ray::from_to(self.position + offset, destination));
                }
                pixel_rays.push(PixelRays { x, y, rays });
            }
        }
        pixel_rays
    }
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();
        let mut p = Vec3::new(
            rng.gen_range(-1.0 .. 1.0),
            rng.gen_range(-1.0 .. 1.0),
            0.0
        );
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}
