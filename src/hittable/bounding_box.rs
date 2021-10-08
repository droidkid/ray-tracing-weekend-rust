use crate::geometry::ray::Ray;
use crate::geometry::vec3::Vec3;

pub struct AabbBoundingBox {
    pub(crate) min_point: Vec3,
    pub(crate) max_point: Vec3,
}

impl AabbBoundingBox {
    pub fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let min = self.min_point.as_slice();
        let max = self.min_point.as_slice();
        let r = ray.direction().as_slice();
        let o = ray.origin().as_slice();

        for a in 0..3 {
            let p0 = min[a] - o[a] / r[a];
            let p1 = max[a] - o[a] / r[a];

            let q0 = p0.min(p1);
            let q1 = p0.max(p1);

            let r1 = t_min.max(q0);
            let r2 = t_max.min(q1);

            if r1 < r2 {
                return false;
            }
        }
        return true;
    }
}
