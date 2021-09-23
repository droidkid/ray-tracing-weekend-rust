use crate::material::color::Color;
use crate::vec3::Vec3;

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color;
}

