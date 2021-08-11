use std::rc::Rc;

use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;

use super::material::Material;

#[derive(Debug, Clone)]
// The normal always points against the incident ray.
// The front_face flag tells us whether we hit the outside or inside.
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub material: Rc<dyn Material>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vector3,
        material: Rc<dyn Material>,
        t: f32,
        u: f32,
        v: f32,
        front_face: bool,
    ) -> Self {
        HitRecord {
            point: point,
            normal: normal,
            material: material,
            t: t,
            u: u,
            v: v,
            front_face: front_face,
        }
    }

    pub fn new_from_incident_ray(
        point: Point3,
        outward_normal: Vector3,
        t: f32,
        u: f32,
        v: f32,
        ray: &Ray,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { outward_normal * -1.0 };

        HitRecord {
            point: point,
            normal: normal,
            material: material,
            t: t,
            u: u,
            v: v,
            front_face: front_face,
        }
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        (self.point == other.point) &&
        (self.normal == other.normal) &&
        (*self.material == *other.material) &&
        (self.t == other.t) &&
        (self.u == other.u) &&
        (self.v == other.v) &&
        (self.front_face == other.front_face)
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::colour::Colour;

    use super::*;
    use super::super::materials::lambertian::Lambertian;

    #[test]
    fn new_hit_record() {
        assert_eq!(
            HitRecord::new(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                7.0,
                0.1,
                0.2,
                true,
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(4.0, 5.0, 6.0),
                material: Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                t: 7.0,
                u: 0.1,
                v: 0.2,
                front_face: true,
            },
        )
    }

    #[test]
    fn new_hit_record_from_incident_ray_same_direction() {
        assert_eq!(
            HitRecord::new_from_incident_ray(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
                0.1,
                0.2,
                &Ray::new(
                    Point3::zero(),
                    Vector3::new(1.0, 0.0, 0.0),
                ),
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(-4.0, -5.0, -6.0),
                material: Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                t: 7.0,
                u: 0.1,
                v: 0.2,
                front_face: false,
            },
        )
    }

    #[test]
    fn new_hit_record_from_incident_ray_opposite_direction() {
        assert_eq!(
            HitRecord::new_from_incident_ray(
                Point3::new(1.0, 2.0, 3.0),
                Vector3::new(4.0, 5.0, 6.0),
                7.0,
                0.1,
                0.2,
                &Ray::new(
                    Point3::zero(),
                    Vector3::new(-1.0, 0.0, 0.0),
                ),
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
            ),
            HitRecord {
                point: Point3::new(1.0, 2.0, 3.0),
                normal: Vector3::new(4.0, 5.0, 6.0),
                material: Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                t: 7.0,
                u: 0.1,
                v: 0.2,
                front_face: true,
            },
        );
    }
}