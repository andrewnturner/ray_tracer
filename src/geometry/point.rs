use std::ops::{Add, Sub, Mul};

use super::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn as_vector3(self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;

    fn add(self, v: Vector3) -> Self {
        Self { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Self;

    fn sub(self, v: Vector3) -> Self {
        Self { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point3() {
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0),
            Point3 { x: 1.0, y: 2.0, z: 3.0 },
        );
    }

    #[test]
    fn zero_point3() {
        assert_eq!(
            Point3::zero(),
            Point3 { x: 0.0, y: 0.0, z: 0.0 },
        );
    }

    #[test]
    fn point3_as_vector3() {
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0).as_vector3(),
            Vector3::new(1.0, 2.0, 3.0),
        )
    }

    #[test]
    fn subtract_point() {
        let p = Point3 { x: 1.0, y: 2.0, z: 3.0 };
        let q = Point3 { x: 2.0, y: 1.0, z: -1.0 };

        assert_eq!(p - q, Vector3 { x: -1.0, y: 1.0, z: 4.0 });
    }

    #[test]
    fn add_vector3() {
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0) + Vector3::new(2.0, 3.0, 4.0),
            Point3::new(3.0, 5.0, 7.0),
        );
    }

    #[test]
    fn subtract_vector3() {
        assert_eq!(
            Point3::new(3.0, 5.0, 7.0) - Vector3::new(1.0, 2.0, 3.0),
            Point3::new(2.0, 3.0, 4.0),
        );
    }

    #[test]
    fn test_multiply_scalar() {
        assert_eq!(
            Point3::new(1.0, 2.0, 3.0) * 2.0,
            Point3::new(2.0, 4.0, 6.0),
        )
    }
}
