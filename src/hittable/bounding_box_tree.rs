use crate::geometry::ray::Ray;
use crate::geometry::vec3::Vec3;
use crate::hittable::bounding_box::AabbBoundingBox;
use crate::hittable::hittable::{HitRecord, Hittable};
use image::hdr::HdrImageDecoderIterator;
use rand::Rng;
use std::cmp::Ordering::Less;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::material::color::Color;
use crate::material::lambertian::Lambertian;
use crate::material::material::Material;

pub static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct BoundingBoxTree {
    aabb_bounding_box: Option<AabbBoundingBox>,
    left: Option<Box<BoundingBoxTree>>,
    right: Option<Box<BoundingBoxTree>>,
    objects: Vec<Arc<Box<dyn Hittable + Send + Sync>>>,
}

impl BoundingBoxTree {
    pub fn new(
        objects: &[Arc<Box<dyn Hittable + Send + Sync>>],
        leaf_size: usize,
    ) -> BoundingBoxTree {
        // Sort by a random axis.
        let mut sorted_objects = vec![];
        for object in objects {
            sorted_objects.push(Arc::clone(&object))
        }
        let mut rng = rand::thread_rng();
        let choose_axis = rng.gen_range(0..3);
        sorted_objects.sort_by(|a, b| {
            a.get_bounding_box().min_point.as_slice()[choose_axis]
                .partial_cmp(&b.get_bounding_box().min_point.as_slice()[choose_axis])
                .unwrap_or(Less)
        });

        if objects.len() <= leaf_size {
            return BoundingBoxTree {
                aabb_bounding_box: Some(build_bounding_box(&sorted_objects)),
                objects: sorted_objects,
                left: None,
                right: None,
            };
        }

        let mid = sorted_objects.len() / 2;
        let left: Option<Box<BoundingBoxTree>> =
            Some(Box::new(BoundingBoxTree::new(&sorted_objects[0..mid], leaf_size)));
        let right: Option<Box<BoundingBoxTree>> =
            Some(Box::new(BoundingBoxTree::new(&sorted_objects[mid..], leaf_size)));

        BoundingBoxTree {
            aabb_bounding_box: Some(build_bounding_box(objects)),
            objects: sorted_objects,
            left,
            right,
        }
    }

    pub fn objects(&self) -> &Vec<Arc<Box<dyn Hittable + Send + Sync>>> {
        &self.objects
    }
}

impl Hittable for BoundingBoxTree {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.get_bounding_box().is_hit(ray, t_min, t_max) {
            return None;
        }

        let mut nearest_hit_record: Option<HitRecord> = None;

        if self.left.is_none() && self.right.is_none() {
            let mut nearest_t = 0.0;
            for object in self.objects.iter() {
                COUNTER.fetch_add(1, Ordering::Relaxed);
                let maybe_hit_record = object.hit(&ray, 0.0001, f64::MAX);
                if maybe_hit_record.is_none() {
                    continue;
                }

                let hit_record = maybe_hit_record.unwrap();
                if nearest_hit_record.is_none() || hit_record.t < nearest_t {
                    nearest_t = hit_record.t;
                    nearest_hit_record = Some(hit_record);
                }
            }
            return nearest_hit_record;
        }

        let left_hit = self.left.as_ref().unwrap().hit(ray, t_min, t_max);
        let right_hit = self.right.as_ref().unwrap().hit(ray, t_min, t_max);

        if left_hit.is_none() && right_hit.is_none() {
            return None;
        }

        if left_hit.is_none() {
            return right_hit;
        }

        if right_hit.is_none() {
            return left_hit;
        }

        let left_hit = left_hit.unwrap();
        let right_hit = right_hit.unwrap();

        return if left_hit.t < right_hit.t {
            Some(left_hit)
        } else {
            Some(right_hit)
        };
    }

    fn get_bounding_box(&self) -> AabbBoundingBox {
        let b = self.aabb_bounding_box.as_ref().unwrap();
        AabbBoundingBox {
            min_point: Vec3::new(b.min_point.x(), b.min_point.y(), b.min_point.z()),
            max_point: Vec3::new(b.max_point.x(), b.max_point.y(), b.max_point.z()),
        }
    }
}

fn build_bounding_box(objects: &[Arc<Box<dyn Hittable + Send + Sync>>]) -> AabbBoundingBox {
    let mut min_x = objects[0].get_bounding_box().min_point.x();
    let mut min_y = objects[0].get_bounding_box().min_point.y();
    let mut min_z = objects[0].get_bounding_box().min_point.z();

    let mut max_x = objects[0].get_bounding_box().max_point.x();
    let mut max_y = objects[0].get_bounding_box().max_point.y();
    let mut max_z = objects[0].get_bounding_box().max_point.z();

    for o in objects {
        min_x = min_x.min(o.get_bounding_box().min_point.x());
        min_y = min_y.min(o.get_bounding_box().min_point.y());
        min_z = min_z.min(o.get_bounding_box().min_point.z());

        max_x = max_x.max(o.get_bounding_box().max_point.x());
        max_y = max_y.max(o.get_bounding_box().max_point.y());
        max_z = max_z.max(o.get_bounding_box().max_point.z());
    }

    AabbBoundingBox {
        min_point: Vec3::new(min_x, min_y, min_z),
        max_point: Vec3::new(max_x, max_y, max_z),
    }
}
