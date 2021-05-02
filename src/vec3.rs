use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
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

    pub fn unit_vector(&self) -> Vec3 {
        return Vec3::new(
            self.x() / self.len(),
            self.y() / self.len(),
            self.z() / self.len(),
        );
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x(), self.y(), self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vec3 {
            x: self.x() * other,
            y: self.y() * other,
            z: self.z() * other,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x() * other,
            y: self.y() * other,
            z: self.z() * other,
        };
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Vec3 {
            x: self.x() * other as f64,
            y: self.y() * other as f64,
            z: self.z() * other as f64,
        }
    }
}

impl ops::MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, other: i32) {
        *self = Self {
            x: self.x() * other as f64,
            y: self.y() * other as f64,
            z: self.z() * other as f64,
        };
    }
}

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
fn add_assign_vector() {
    let mut point1 = Vec3::new(1.0, -2.0, 7.3);
    let point2 = Vec3::new(1.0, -2.0, 7.3);
    point1 += point2;
    assert_eq!(point1.x(), 2.0);
    assert_eq!(point1.y(), -4.0);
    assert_eq!(point1.z(), 14.6);
}

#[test]
fn mul_vector_f64() {
    let point1 = Vec3::new(1.0, -2.0, 7.0);
    let point2 = point1 * 2.0;
    assert_eq!(point2.x(), 2.0);
    assert_eq!(point2.y(), -4.0);
    assert_eq!(point2.z(), 14.0);
}

#[test]
fn mul_assign_vector_f64() {
    let mut point = Vec3::new(1.0, -2.0, 7.0);
    point *= 0.5;
    assert_eq!(point.x(), 0.5);
    assert_eq!(point.y(), -1.0);
    assert_eq!(point.z(), 3.5);
}

#[test]
fn mul_vector_i32() {
    let point1 = Vec3::new(1.0, -2.0, 7.0);
    let point2 = point1 * 2;
    assert_eq!(point2.x(), 2.0);
    assert_eq!(point2.y(), -4.0);
    assert_eq!(point2.z(), 14.0);
}

#[test]
fn mul_assign_vector_i32() {
    let mut point = Vec3::new(1.0, -2.0, 7.0);
    point *= 4;
    assert_eq!(point.x(), 4.0);
    assert_eq!(point.y(), -8.0);
    assert_eq!(point.z(), 28.0);
}
