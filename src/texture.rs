use crate::color::Color;
use crate::vec3::Vec3;

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color;
}

pub struct SolidColorTexture {
    color: Color
}

pub struct CheckeredTexture {
    evenColor: Color,
    oddColor: Color,
    sizeFactor: f64
}

impl SolidColorTexture {
    pub fn new(color: Color) -> SolidColorTexture {
        SolidColorTexture {
            color
        }
    }
}

impl Texture for SolidColorTexture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color {
        self.color
    }
}

impl CheckeredTexture{
    pub fn new(evenColor: Color, oddColor: Color, sizeFactor: f64) -> CheckeredTexture {
        CheckeredTexture {
            evenColor,
            oddColor,
            sizeFactor
        }
    }
}

impl Texture for CheckeredTexture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color {
        let sines = (point.x() / self.sizeFactor).sin() * (point.y() / self.sizeFactor).sin() * (point.z() / self.sizeFactor).sin();
        if sines < 0.0 {
            self.oddColor
        } else {
            self.evenColor
        }
    }
}