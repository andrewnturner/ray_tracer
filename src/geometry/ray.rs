use super::point::Point3;
use num_traits::float::Float;

use super::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray<T: Float> {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
}

impl<T: Float> Ray<T> {
    pub fn new(origin: Point3<T>, direction: Vector3<T>) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + (self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ray() {
        assert_eq!(
            Ray::new(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
            ),
            Ray {
                origin: Point3 { x: 1.0, y: 2.0, z: 3.0 },
                direction: Vector3 { x: 4.0, y: 5.0, z: 6.0 },
            },
        );
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(
            Point3::new(1.0, 2.0, 3.0),
            Vector3::new(4.0, 5.0, 6.0),
        );

        assert_eq!(
            ray.at(2.0),
            Point3::new(9.0, 12.0, 15.0),
        );
    }
}
