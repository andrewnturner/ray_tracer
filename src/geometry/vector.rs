use num_traits::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Self {
        Vector3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn length_squared(&self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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
}
