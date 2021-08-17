use std::fmt;
use std::ops;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn random(min: f64, max: f64)  -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max)
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn len_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        return Vec3::new(
            self.x() / self.len(),
            self.y() / self.len(),
            self.z() / self.len(),
        );
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.normalize()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * dot(self, normal) * normal
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x(), self.y(), self.z())
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3 | -> Vec3 {
    Vec3 {
        x: a.x() + b.x(),
        y: a.y() + b.y(),
        z: a.z() + b.z()
    }
});

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x() - b.x(),
        y: a.y() - b.y(),
        z: a.z() - b.z(),
    }
});

impl_op!(*|a: Vec3, b: f64| -> Vec3 {
    Vec3 {
        x: a.x() * b,
        y: a.y() * b,
        z: a.z() * b,
    }
});

impl_op!(*|a: &Vec3, b: f64| -> Vec3 {
    Vec3 {
        x: a.x() * b,
        y: a.y() * b,
        z: a.z() * b,
    }
});

impl_op!(*|b: f64, a: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x() * b,
        y: a.y() * b,
        z: a.z() * b,
    }
});

impl_op!(*|b: f64, a: Vec3| -> Vec3 {
    Vec3 {
        x: a.x() * b,
        y: a.y() * b,
        z: a.z() * b,
    }
});

#[test]
fn origin_vector() {
    let origin = Vec3::origin();
    assert_eq!(origin.x(), 0.0);
    assert_eq!(origin.y(), 0.0);
    assert_eq!(origin.z(), 0.0);
}

#[test]
fn unit_vector() {
    let vec = Vec3::new(3.0, 0.0, 0.0);
    let unit_vec = vec.unit_vector();
    assert_eq!(unit_vec.x(), 1.0);
    assert_eq!(unit_vec.y(), 0.0);
    assert_eq!(unit_vec.z(), 0.0);
}

#[test]
fn point_vector() {
    let point = Vec3::new(1.0, -2.0, 7.3);
    assert_eq!(point.x(), 1.0);
    assert_eq!(point.y(), -2.0);
    assert_eq!(point.z(), 7.3);
}

#[test]
fn add_vector() {
    let point1 = Vec3::new(1.0, -2.0, 7.3);
    let point2 = Vec3::new(1.0, -2.0, 7.3);
    let point = point1 + point2;
    assert_eq!(point.x(), 2.0);
    assert_eq!(point.y(), -4.0);
    assert_eq!(point.z(), 14.6);
}

#[test]
fn mul_vector() {
    let point1 = Vec3::new(1.0, -2.0, 7.0);
    let point2 = point1 * 2.5;
    assert_eq!(point2.x(), 2.5);
    assert_eq!(point2.y(), -5.0);
    assert_eq!(point2.z(), 17.5);
}
