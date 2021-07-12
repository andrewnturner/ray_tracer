use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
// The normal always points against the incident ray.
// The front_face flag tells us whether we hit the outside or inside.
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vector3, t: f32, front_face: bool) -> Self {
        HitRecord {
            point: point,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }

    pub fn new_from_incident_ray(point: Point3, outward_normal: Vector3, t: f32, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { outward_normal * -1.0 };

        HitRecord {
            point: point,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_hit_record() {
        assert_eq!(
            HitRecord::new(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
                true,
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(4.0, 5.0, 6.0),
                t: 7.0,
                front_face: true
            }
        )
    }

    #[test]
    fn new_hit_record_from_incident_ray_same_direction() {
        assert_eq!(
            HitRecord::new_from_incident_ray(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
                &Ray::new(
                    Point3::zero(),
                    Vector3::new(1.0, 0.0, 0.0),
                ),
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(-4.0, -5.0, -6.0),
                t: 7.0,
                front_face: false,
            }
        )
    }

    #[test]
    fn new_hit_record_from_incident_ray_opposite_direction() {
        assert_eq!(
            HitRecord::new_from_incident_ray(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
                &Ray::new(
                    Point3::zero(),
                    Vector3::new(-1.0, 0.0, 0.0),
                ),
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(4.0, 5.0, 6.0),
                t: 7.0,
                front_face: true,
            }
        )
    }
}