use crate::geometry::vec3::Vec3;

pub struct BoundingBox {
    pub(crate) min_point: Vec3,
    pub(crate) max_point: Vec3,
}
