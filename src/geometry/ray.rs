use super::point::Point3;

use super::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Ray {
            origin: origin,
            direction: direction,
            time: 0.0,
        }
    }

    pub fn new_at_time(origin: Point3, direction: Vector3, time: f32) -> Self {
        Ray {
            origin: origin,
            direction: direction,
            time: time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
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
                time: 0.0,
            },
        );
    }

    #[test]
    fn new_ray_at_time2() {
        assert_eq!(
            Ray::new_at_time(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                1.0,
            ),
            Ray {
                origin: Point3 { x: 1.0, y: 2.0, z: 3.0 },
                direction: Vector3 { x: 4.0, y: 5.0, z: 6.0 },
                time: 1.0,
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
