use crate::vec3::Vec3;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    rgb: Vec3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        if r < 0.0 || r > 1.0 {
            panic!("Not a valid color")
        }
        if g < 0.0 || b > 1.0 {
            panic!("Not a valid color")
        }
        if g < 0.0 || b > 1.0 {
            panic!("Not a valid color")
        }

        Color {
            rgb: Vec3::new(r, g, b),
        }
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn random() -> Color {
        Color {
            rgb: Vec3::random(0.0, 1.0),
        }
    }
    pub fn r(&self) -> f64 {
        self.rgb.x()
    }

    pub fn g(&self) -> f64 {
        self.rgb.y()
    }

    pub fn b(&self) -> f64 {
        self.rgb.z()
    }

    pub fn new_from_vector(rgb: Vec3) -> Color {
        Color { rgb }
    }

    pub fn lerp(start_color: Color, end_color: Color, param: f64) -> Color {
        Color {
            rgb: start_color.rgb * (1.0 - param) + end_color.rgb * param,
        }
    }

    pub fn attenuate(&self, factor: Color) -> Color {
        Color {
            rgb: Vec3::new(
                self.r() * factor.r(),
                self.g() * factor.g(),
                self.b() * factor.b(),
            ),
        }
    }

    pub fn image_pixel(&self) -> image::Rgb<u8> {
        let r = self.r() * 256.0;
        let g = self.g() * 256.0;
        let b = self.b() * 256.0;
        image::Rgb([r as u8, g as u8, b as u8])
    }

    pub fn gamma_corrected(&self) -> Color {
        Color::new(
            self.r().sqrt(),
            self.g().sqrt(),
            self.b().sqrt(),
        )
    }

    pub fn average_color<'a>(colors: impl Iterator<Item = &'a Color>) -> Color {
        let mut total_rgb = Vec3::origin();
        let mut count = 0.0;
        for color in colors {
            total_rgb = total_rgb + color.rgb;
            count = count + 1.0
        }
        Color::new(
            total_rgb.x() / count,
            total_rgb.y() / count,
            total_rgb.z() / count,
        )
    }

    pub fn as_vector(&self) -> Vec3 {
        self.rgb
    }
}

impl_op_ex!(+ |a: &Color, b: &Color | -> Color {
    let rgb = a.rgb + b.rgb;
    let rgb = Vec3::new(
        rgb.x().clamp(0.0, 256.0),
        rgb.y().clamp(0.0, 256.0),
        rgb.z().clamp(0.0, 256.0),
    );
    Color {
        rgb
    }
});
