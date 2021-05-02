extern crate image;

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    // Image.
    let img_width = 400;
    let img_height = (img_width as f64 / aspect_ratio) as u32;
    let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    // Camera.
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    // Eye is situated at origin.
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
        let t = 0.5 * (unit_direction.y() + 1.0);

        let white = Vec3::new(1.0, 1.0, 1.0);
        let blueish = Vec3::new(0.5, 0.7, 1.0);
        let color = white * (1.0 - t) + blueish * t;

        let r = (color.x() * 256.0) as u8;
        let g = (color.y() * 256.0) as u8;
        let b = (color.z() * 256.0) as u8;

        *pixel = image::Rgb([r, g, b]);
    }

    img_buf.save("gradient.png").unwrap();
}
