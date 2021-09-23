use crate::material::color::Color;
use crate::material::texture::Texture;
use crate::vec3::Vec3;

pub struct CheckeredTexture {
    even_color: Color,
    odd_color: Color,
    size_factor: f64,
}

impl CheckeredTexture {
    pub fn new(even_color: Color, odd_color: Color, size_factor: f64) -> CheckeredTexture {
        CheckeredTexture {
            even_color,
            odd_color,
            size_factor,
        }
    }
}

impl Texture for CheckeredTexture {
    fn get_color(&self, _u: f64, _v: f64, point: Vec3) -> Color {
        let sines = (point.x() / self.size_factor).sin()
            * (point.y() / self.size_factor).sin()
            * (point.z() / self.size_factor).sin();
        if sines < 0.0 {
            self.odd_color
        } else {
            self.even_color
        }
    }
}
