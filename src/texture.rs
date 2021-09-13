use crate::color::Color;
use crate::vec3::Vec3;

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color;
}

pub struct SolidColorTexture {
    color: Color,
}

pub struct CheckeredTexture {
    even_color: Color,
    odd_color: Color,
    size_factor: f64,
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
