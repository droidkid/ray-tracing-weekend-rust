use crate::vec3::Vec3;

pub struct Color {
    rgb: Vec3,
}

impl Color {
    pub fn new(r: f64, g:f64, b:f64)  -> Color {
        Color {
            // TODO(chesetti): Do something if any component greater than 1
            rgb: Vec3::new(r, g, b)
        }
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn lerp(start_color: Color, end_color: Color, param: f64) -> Color {
        Color {
            rgb: start_color.rgb * (1.0 - param) + end_color.rgb * param
        }
    }

    pub fn intensity(&self, factor:f64) -> Color {
        // TODO(chesetti): What happens if factor > 1?
        Color {
            rgb: factor * self.rgb
        }
    }

    pub fn image_pixel(&self) -> image::Rgb<u8> {
        let r = self.rgb.x() * 256.0;
        let g = self.rgb.y() * 256.0;
        let b = self.rgb.z() * 256.0;
        image::Rgb([r as u8, g as u8, b as u8])
    }

    pub fn gamma_corrected(&self) -> Color {
        Color::new(self.rgb.x().sqrt(), self.rgb.y().sqrt(), self.rgb.z().sqrt())
    }

    pub fn average_color<'a>(colors: impl Iterator<Item = &'a Color>) -> Color {
        let mut total_rgb = Vec3::origin();
        let mut count = 0.0;
        for color in colors {
            total_rgb = total_rgb + color.rgb;
            count = count + 1.0
        }
        Color::new(total_rgb.x() / count, total_rgb.y() / count, total_rgb.z() / count)
    }
}