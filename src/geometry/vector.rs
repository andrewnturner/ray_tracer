use std::ops::{Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalise(&self) -> Self {
        let length = self.length();

        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        debug_assert!(scalar != 0.0);
        let inv = scalar.recip();

        Self { x: self.x * inv, y: self.y * inv, z: self.z * inv }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector3() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0),
            Vector3 { x: 1.0, y: 2.0, z: 3.0 },
        );
    }

    #[test]
    fn zero_vector3() {
        assert_eq!(
            Vector3::zero(),
            Vector3 { x: 0.0, y: 0.0, z: 0.0 },
        );
    }

    #[test]
    fn length_squared_vector3() {
        let v = Vector3 { x: 2.0, y: -10.0, z: 11.0 };

        assert_eq!(v.length_squared(), 225.0);
    }

    #[test]
    fn length_vector3() {
        let v = Vector3 { x: 1.0, y: 2.0, z: -2.0 };

        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn normalise_vector3() {
        let v = Vector3 { x: 1.0, y: 1.0, z: 1.0 };

        let n = 1.0 / 3.0_f32.sqrt();
        assert_eq!(v.normalise(), Vector3 { x: n, y: n, z: n });
    }

    #[test]
    fn dot_vector3() {
        let v = Vector3 { x: -1.0, y: -2.0, z: -3.0 };
        let w = Vector3 { x: 3.0, y: 4.0, z: 5.0 };

        assert_eq!(v.dot(&w), -26.0);
    }

    #[test]
    fn test_multiply_scalar() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) * 2.0,
            Vector3::new(2.0, 4.0, 6.0),
        )
    }

    #[test]
    fn test_divide_scalar() {
        assert_eq!(
            Vector3::new(1.0, 2.0, 3.0) / 2.0,
            Vector3::new(0.5, 1.0, 1.5),
        )
    }
}
