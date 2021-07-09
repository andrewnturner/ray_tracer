use num_traits::float::Float;

use crate::geometry::point::Point3;
use crate::geometry::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HitRecord<T: Float> {
    pub point: Point3<T>,
    pub normal: Vector3<T>,
    pub t: T
}

impl<T: Float> HitRecord<T> {
    pub fn new(point: Point3<T>, normal: Vector3<T>, t: T) -> Self {
        HitRecord {
            point: point,
            normal: normal,
            t: t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_hit_records() {
        assert_eq!(
            HitRecord::new(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(4.0, 5.0, 6.0),
                t: 7.0,
            }
        )
    }
}