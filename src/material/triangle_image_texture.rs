use crate::geometry::vec3::Vec3;
use crate::material::color::Color;
use crate::material::image_texture::ImageTexture;
use crate::material::texture::Texture;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Pixel};

pub struct TriangleImageTexture {
    // TODO(chesetti): Make this a shared image.
    img: DynamicImage,
    v1: Vec3,
    v2: Vec3,
    p1: Vec3,
}

impl TriangleImageTexture {
    pub fn new(path: &str, p1: Vec3, p2: Vec3, p3: Vec3) -> TriangleImageTexture {
        TriangleImageTexture {
            img: ImageReader::open(path).unwrap().decode().unwrap(),
            v1: p2 - p1,
            v2: p3 - p1,
            p1,
        }
    }
}

impl Texture for TriangleImageTexture {
    fn get_color(&self, u: f64, v: f64, _point: Vec3) -> Color {
        let p = u * self.v1 + v * self.v2 + self.p1;

        let x = p.x() as u32;
        let y = p.y() as u32;

        let color = self.img.get_pixel(x, y).to_rgb();

        Color::new(
            (color[0] as f64 / 256.0),
            color[1] as f64 / 256.0,
            color[2] as f64 / 256.0,
        )
    }
}
