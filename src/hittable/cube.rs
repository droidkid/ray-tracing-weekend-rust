use crate::geometry::ray::Ray;
use crate::geometry::vec3::{cross, Vec3};
use crate::hittable::bounding_box::AabbBoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use crate::hittable::triangle::Triangle;
use crate::material::color::Color;
use crate::material::lambertian::Lambertian;
use crate::material::material::Material;
use crate::material::metal::Metal;
use crate::material::texture::Texture;
use crate::material::triangle_image_texture::TriangleImageTexture;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::sync::Arc;

pub struct Cube {
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    points: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl Cube {
    pub fn new(
        center: Vec3,
        scale: f64,
        to: Vec3,
        material: Arc<Box<dyn Material + Send + Sync>>,
    ) -> Cube {
        // TODO(chesetti): Add rotation around (center - to)
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (center - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let forward = forward * scale;
        let right = right * scale;
        let up = up * scale;

        let points = build_points(forward, right, up, center);
        let triangles = build_same_material(&points, material);

        Cube {
            forward,
            right,
            up,
            triangles,
            points,
        }
    }

    pub fn newCuboid(
        center: Vec3,
        to: Vec3,
        width: f64,
        height: f64,
        depth: f64,
        material: Arc<Box<dyn Material + Send + Sync>>,
    ) -> Cube {
        // TODO(chesetti): Add rotation around (center - to)
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (center - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let forward = forward * (depth * 0.5);
        let right = right * (width * 0.5);
        let up = up * (height * 0.5);

        let points = build_points(forward, right, up, center);
        let triangles = build_same_material(&points, material);

        Cube {
            forward,
            right,
            up,
            triangles,
            points,
        }
    }

    pub fn new_mapped_cube(
        img_path: &str,
        center: Vec3,
        to: Vec3,
        width: f64,
        height: f64,
        depth: f64,
    ) -> Cube {
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let forward = (center - to).normalize();
        let right: Vec3 = cross(&vup, &forward).normalize();
        let up: Vec3 = cross(&forward, &right).normalize();

        let forward = forward * (depth * 0.5);
        let right = right * (width * 0.5);
        let up = up * (height * 0.5);

        let points = build_points(forward, right, up, center);
        let triangles = build_die_material(img_path, &points);

        Cube {
            forward,
            right,
            up,
            triangles,
            points,
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut nearest_hit_record: Option<HitRecord> = None;
        let mut nearest_t = 0.0;

        for triangle in self.triangles.iter() {
            let maybe_hit_record = triangle.hit(&ray, t_min, t_max);
            if maybe_hit_record.is_none() {
                continue;
            }

            let hit_record = maybe_hit_record.unwrap();
            if nearest_hit_record.is_none() || hit_record.t < nearest_t {
                nearest_t = hit_record.t;
                nearest_hit_record = Some(hit_record);
            }
        }
        nearest_hit_record
    }

    fn get_bounding_box(&self) -> AabbBoundingBox {
        let mut min_x = self.points[0].x();
        let mut min_y = self.points[0].y();
        let mut min_z = self.points[0].z();

        let mut max_x = self.points[0].x();
        let mut max_y = self.points[0].y();
        let mut max_z = self.points[0].z();

        for p in &self.points {
            min_x = min_x.min(p.x());
            min_y = min_y.min(p.y());
            min_z = min_z.min(p.z());

            max_x = max_x.max(p.x());
            max_y = max_y.max(p.y());
            max_z = max_z.max(p.z());
        }

        AabbBoundingBox {
            min_point: Vec3::new(min_x, min_y, min_z),
            max_point: Vec3::new(max_x, max_y, max_z),
        }
    }
}

fn build_triangle(
    points: &Vec<Vec3>,
    p1: usize,
    p2: usize,
    p3: usize,
    material: &Arc<Box<dyn Material + Send + Sync>>,
) -> Triangle {
    Triangle::new(
        points[p1 - 1],
        points[p2 - 1],
        points[p3 - 1],
        Arc::clone(&material),
    )
}

fn build_same_material(
    points: &Vec<Vec3>,
    material: Arc<Box<dyn Material + Send + Sync>>,
) -> Vec<Triangle> {
    vec![
        // Face 1
        build_triangle(&points, 1, 2, 3, &material),
        build_triangle(&points, 4, 2, 3, &material),
        // Face 2
        build_triangle(&points, 5, 6, 7, &material),
        build_triangle(&points, 8, 6, 7, &material),
        // Face 3
        build_triangle(&points, 6, 2, 8, &material),
        build_triangle(&points, 4, 2, 8, &material),
        // Face 4
        build_triangle(&points, 1, 5, 3, &material),
        build_triangle(&points, 7, 5, 3, &material),
        // Face 5
        build_triangle(&points, 1, 5, 2, &material),
        build_triangle(&points, 6, 5, 2, &material),
        // Face 6
        build_triangle(&points, 3, 7, 4, &material),
        build_triangle(&points, 8, 7, 4, &material),
    ]
}

fn build_points(forward: Vec3, right: Vec3, up: Vec3, center: Vec3) -> Vec<Vec3> {
    vec![
        center + (forward + right + up),
        center + (forward + right - up),
        center + (forward - right + up),
        center + (forward - right - up),
        center + ((-1.0 * forward) + right + up),
        center + ((-1.0 * forward) + right - up),
        center + ((-1.0 * forward) - right + up),
        center + ((-1.0 * forward) - right - up),
    ]
}

fn build_uv_mapped_triangle(
    img_path: &str,
    x: u32,
    y: u32,
    is_bot: bool,
) -> Arc<Box<dyn Material + Send + Sync>> {
    // TODO(chesetti): Send reference to image instead of reading everytime?
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();

    let sq_width = width / 4;
    let sq_height = height / 3;

    // The triangles are all right angled.
    let xmin = (x * sq_width) as f64;
    let ymin = (y * sq_height) as f64;

    let xmax = ((x + 1) * sq_width) as f64;
    let ymax = ((y + 1) * sq_height) as f64;

    let triangle_texture: Box<dyn Texture + Send + Sync>;
    if is_bot {
        triangle_texture = Box::new(TriangleImageTexture::new(
            img_path,
            Vec3::new(xmin, ymax, 0.0),
            Vec3::new(xmax, ymax, 0.0),
            Vec3::new(xmin, ymin, 0.0),
        ));
    } else {
        triangle_texture = Box::new(TriangleImageTexture::new(
            img_path,
            Vec3::new(xmax, ymin, 0.0),
            Vec3::new(xmax, ymax, 0.0),
            Vec3::new(xmin, ymin, 0.0),
        ));
    }

    Arc::new(Box::new(Lambertian::new_from_texture(triangle_texture)))
}

fn build_die_material(img_path: &str, points: &Vec<Vec3>) -> Vec<Triangle> {
    vec![
        // Face 1
        build_triangle(
            &points,
            1,
            3,
            2,
            &build_uv_mapped_triangle(img_path, 0, 1, false),
        ),
        build_triangle(
            &points,
            4,
            3,
            2,
            &build_uv_mapped_triangle(img_path, 0, 1, true),
        ),
        // Face 2
        build_triangle(
            &points,
            7,
            5,
            8,
            &build_uv_mapped_triangle(img_path, 1, 1, false),
        ),
        build_triangle(
            &points,
            6,
            5,
            8,
            &build_uv_mapped_triangle(img_path, 1, 1, true),
        ),
        // Face 3
        build_triangle(
            &points,
            6,
            2,
            8,
            &build_uv_mapped_triangle(img_path, 2, 0, false),
        ),
        build_triangle(
            &points,
            4,
            2,
            8,
            &build_uv_mapped_triangle(img_path, 2, 0, true),
        ),
        // Face 4
        build_triangle(
            &points,
            5,
            1,
            7,
            &build_uv_mapped_triangle(img_path, 2, 2, false),
        ),
        build_triangle(
            &points,
            3,
            1,
            7,
            &build_uv_mapped_triangle(img_path, 2, 2, true),
        ),
        // Face 5
        build_triangle(
            &points,
            5,
            1,
            6,
            &build_uv_mapped_triangle(img_path, 2, 1, false),
        ),
        build_triangle(
            &points,
            2,
            1,
            6,
            &build_uv_mapped_triangle(img_path, 2, 1, true),
        ),
        // Face 6
        build_triangle(
            &points,
            3,
            7,
            4,
            &build_uv_mapped_triangle(img_path, 3, 1, false),
        ),
        build_triangle(
            &points,
            8,
            7,
            4,
            &build_uv_mapped_triangle(img_path, 3, 1, true),
        ),
    ]
}
