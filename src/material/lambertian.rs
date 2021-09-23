use crate::material::color::Color;
use crate::hittable::HitRecord;
use crate::material::material::{Material, ScatterResult};
use crate::ray::Ray;
use crate::material::texture:: Texture;
use crate::vec3::Vec3;
use crate::material::solid_color_texture::SolidColorTexture;

pub struct Lambertian {
    texture: Box<dyn Texture + Send + Sync>,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            texture: Box::new(SolidColorTexture::new(Color::new_from_vector(albedo))),
        }
    }

    pub fn new_from_texture(texture: Box<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        ScatterResult {
            scattered_ray: Some(Ray::new(hit_record.hit_point, scatter_direction)),
            attenuation: self
                .texture
                .get_color(hit_record.u, hit_record.v, hit_record.hit_point),
            emitted: Color::black(),
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.len_squared() < 1.0 {
            return p;
        }
    }
}
