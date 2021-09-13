use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};
use rand::Rng;
use crate::color::Color;
use crate::texture::{SolidColorTexture, Texture};

pub struct ScatterResult {
    pub scattered_ray: Option<Ray>,
    // TODO(chesetti): Should attenuation be color?
    pub attenuation: Vec3,
    pub emitted: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult;
}

pub struct Lambertian {
    texture: Box<dyn Texture + Send + Sync>,
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

pub struct Dielectric {
    index_of_refraction: f64,
}

pub struct DiffuseLight {
    emit_color: Color,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            texture: Box::new(SolidColorTexture::new(Color::new_from_vector(albedo))),
        }
    }

    pub fn new_from_texture(texture: Box<dyn Texture + Send + Sync>) -> Lambertian {
        Lambertian {
            texture
        }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fz: f64) -> Metal {
        let mut fuzz = fz;

        if fz > 1.0 {
            fuzz = 1.0;
        }
        if fz < 0.0 {
            fuzz = 0.0;
        }

        Metal { albedo, fuzz }
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction,
        }
    }
}

impl DiffuseLight {
    pub fn new(emit_color: Color) -> DiffuseLight {
        DiffuseLight {
            emit_color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        ScatterResult {
            scattered_ray: Some(Ray::new(hit_record.hit_point, scatter_direction)),
            attenuation: self.texture.get_color(hit_record.u, hit_record.v, hit_record.hit_point).as_vector(),
            emitted: Color::black()
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let unit_vector = ray_in.direction().unit_vector();
        let scatter_direction =
            unit_vector.reflect(&hit_record.normal) + self.fuzz * random_in_unit_sphere();
        let scattered_ray = Ray::new(hit_record.hit_point, scatter_direction);
        if dot(scattered_ray.direction(), &hit_record.normal) > 0.0 {
            ScatterResult {
                scattered_ray: Some(scattered_ray),
                attenuation: self.albedo,
                emitted: Color::black()
            }
        } else {
            ScatterResult {
                scattered_ray: None,
                attenuation: self.albedo,
                emitted: Color::black()
            }
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.index_of_refraction,
            false => self.index_of_refraction,
        };

        fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
            let cos = (dot(&(uv * -1.0), normal)).min(1.0);
            let r_out_perp = etai_over_etat * (uv + cos * normal);
            let r_out_parallel = -1.0 * ((1.0 - r_out_perp.len_squared()).sqrt()) * normal;
            r_out_perp + r_out_parallel
        }

        fn reflectance(cos: f64, ref_idx: f64) -> f64 {
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0 * r0;
            return r0 + (1.0 - r0) * ((1.0 - cos).powf(5.0));
        }

        let unit_direction = ray_in.direction().normalize();
        let cos = (dot(&(unit_direction * -1.0), &hit_record.normal)).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = (refraction_ratio * sin) > 1.0;
        let direction;

        if cannot_refract || reflectance(cos, refraction_ratio) > random_double() {
            direction = unit_direction.reflect(&hit_record.normal);
        } else {
            direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }
        ScatterResult {
            scattered_ray: Some(Ray::new(hit_record.hit_point, direction)),
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            emitted: Color::black()
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> ScatterResult {
        ScatterResult {
            scattered_ray: None,
            attenuation: Vec3::new(0.0, 0.0, 0.0),
            emitted: self.emit_color
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
fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
