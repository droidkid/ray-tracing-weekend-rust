extern crate image;

mod ray;
mod vec3;

fn main() {
    let img_width = 256;
    let img_height = 256;

    let mut img_buf = image::ImageBuffer::new(img_width, img_height);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        if y < 100 {
            continue;
        } else {
            let r = ((x as f64) / ((img_width - 1) as f64) * 256.0) as u8;
            let g = ((y as f64) / ((img_height - 1) as f64) * 256.0) as u8;
            *pixel = image::Rgb([r, g, 64])
        }
    }

    img_buf.save("gradient.png").unwrap();
}
