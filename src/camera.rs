use crate::ray::Ray;
use crate::vec3::{cross, Vec3};

pub struct Camera {
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,

    // Viewport Stuff
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
        viewport_width: f64,
        viewport_height: f64,
        raster_width: u32,
        raster_height: u32,
    ) -> Camera {
        // Reference: https://www.scratchapixel.com/lessons/mathematics-physics-for-computer-graphics/lookat-function
        let forward = (from - to).normalize();
        let right: Vec3 = cross(&Vec3::new(0.0, 1.0, 0.0), &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();
        Camera {
            position: from,
            forward,
            right,
            up,
            focal_length: (to - from).len(),
            viewport_width,
            viewport_height,
            raster_width,
            raster_height,
        }
    }

    // Function that returns a list of rays for each pixel in the raster
    pub fn get_rays(&self) -> Vec<PixelRays> {
        let viewport_center = self.position + self.forward * (-1.0 * self.focal_length);
        let viewport_upper_left = viewport_center
            + ((-0.5 * self.viewport_height) * self.up)
            + ((-0.5 * self.viewport_width) * self.right);

        let mut pixel_rays: Vec<PixelRays> = vec![];

        for x in 0..self.raster_width {
            for y in 0..self.raster_height {
                let delta = Vec3::new(
                    self.viewport_width * (x as f64 / self.raster_width as f64),
                    self.viewport_height * ((self.raster_height - y) as f64 / self.raster_height as f64),
                    0.0,
                );

                let destination = viewport_upper_left + delta;

                pixel_rays.push(PixelRays {
                    x,
                    y,
                    rays: vec![Ray::new(self.position, destination)],
                });
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
