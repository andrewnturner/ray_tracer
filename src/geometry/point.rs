use std::ops::{Add, Sub};

use num_traits::float::Float;

use super::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Point3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn as_vector3(self) -> Vector3<T> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl <T: Float> Sub for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, other: Self) -> Vector3<T> {
        Vector3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl <T: Float> Add<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, v: Vector3<T>) -> Self {
        Self { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }
}

impl <T: Float> Sub<Vector3<T>> for Point3<T> {
    type Output = Self;

    fn sub(self, v: Vector3<T>) -> Self {
        Self { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
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
}
