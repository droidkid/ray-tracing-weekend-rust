use crate::geometry::vec3::Vec3;
use crate::material::color::Color;
use crate::material::texture::Texture;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;

pub struct ImageTexture {
    img: DynamicImage,
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture {
        ImageTexture {
            img: ImageReader::open(path).unwrap().decode().unwrap(),
        }
    }
}

impl Texture for ImageTexture {
    fn get_color(&self, u: f64, v: f64, _point: Vec3) -> Color {
        let x = (u * self.img.width() as f64) as u32;
        let y = (v * self.img.height() as f64) as u32;

        let color = self.img.get_pixel(x, y).to_rgb();

        Color::new(
            (color[0] as f64 / 256.0),
            color[1] as f64 / 256.0,
            color[2] as f64 / 256.0,
        )
    }
}
