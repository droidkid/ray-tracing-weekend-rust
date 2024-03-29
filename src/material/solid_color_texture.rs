use crate::geometry::vec3::Vec3;
use crate::material::color::Color;
use crate::material::texture::Texture;

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> SolidColorTexture {
        SolidColorTexture { color }
    }
}

impl Texture for SolidColorTexture {
    fn get_color(&self, _u: f64, _v: f64, _point: Vec3) -> Color {
        self.color
    }
}
