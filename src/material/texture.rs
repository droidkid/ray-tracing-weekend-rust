use crate::geometry::vec3::Vec3;
use crate::material::color::Color;

pub trait Texture {
    fn get_color(&self, u: f64, v: f64, point: Vec3) -> Color;
}
