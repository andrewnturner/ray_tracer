use std::any::Any;
use std::fmt::Debug;
use std::rc::Rc;

use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;

use super::super::element::Element;
use super::super::hit_record::HitRecord;
use super::super::material::Material;

#[derive(Debug, Clone)]
pub struct MovingSphere {
    pub centre_0: Point3,
    pub centre_1: Point3,
    pub time_0: f32,
    pub time_1: f32,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        centre_0: Point3,
        centre_1: Point3,
        time_0: f32,
        time_1: f32,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        MovingSphere {
            centre_0: centre_0,
            centre_1: centre_1,
            time_0: time_0,
            time_1: time_1,
            radius: radius,
            material: material,
        }
    }

    fn centre_at(&self, time: f32) -> Point3 {
        let fraction_travelled = (time - self.time_0) / (self.time_1 - self.time_0);

        self.centre_0 + ((self.centre_1 - self.centre_0) * fraction_travelled)
    }
}

impl Element for MovingSphere {

    /// We have a ray R(t) = A + tb and sphere of radius r centred at C. We have an intersection
    /// if there is t such that
    ///     (R(t) - C) . (R(t) - C) = r^2.
    /// Expanding gives
    ///     (t^2)(b . b) + 2t(b . (A - C)) + ((A - C) . A - C) - r^2 = 0,
    /// a quadratic in t. We can then look at the discriminant ot see whether there are
    /// any solutions.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.centre_at(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return None
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = ((half_b * -1.0) - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = ((half_b * -1.0) + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None
            }
        }

        let p = ray.at(root);
        let normal = (p - self.centre_at(ray.time)) / self.radius;

        Some(HitRecord::new_from_incident_ray(
            p,
            normal,
            root,
            &ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<BoundingBox> {
        let box_0 = BoundingBox::new(
            self.centre_at(time_0) - Vector3::new(self.radius, self.radius, self.radius),
            self.centre_at(time_0) + Vector3::new(self.radius, self.radius, self.radius),
        );
        let box_1 = BoundingBox::new(
            self.centre_at(time_1) - Vector3::new(self.radius, self.radius, self.radius),
            self.centre_at(time_1) + Vector3::new(self.radius, self.radius, self.radius),
        );

        Some(box_0.union(&box_1))
    }

    fn eq(&self, other: &dyn Element) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for MovingSphere {
    fn eq(&self, other: &Self) -> bool {
        (self.centre_0 == other.centre_0) &&
        (self.centre_1 == other.centre_1) &&
        (self.time_0 == other.time_0) &&
        (self.time_1 == other.time_1) &&
        (self.radius == other.radius) && 
        (*self.material == *other.material)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector3;
    use crate::graphics::colour::Colour;

    use super::*;
    use super::super::super::materials::lambertian::Lambertian;

    #[test]
    fn new_moving_sphere() {
        assert_eq!(
            MovingSphere::new(
                Point3::new(1.0, 2.0, 3.0),
                Point3::new(4.0, 5.0, 6.0),
                0.0,
                1.0,
                5.0,
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
            ),
            MovingSphere {
                centre_0: Point3 { x: 1.0, y: 2.0, z: 3.0 },
                centre_1: Point3 { x: 4.0, y: 5.0, z: 6.0 },
                time_0: 0.0,
                time_1: 1.0,
                radius: 5.0,
                material: Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
            },
        )
    }

    #[test]
    fn hit_moving_sphere() {
        let ray = Ray::new_at_time(Point3::zero(), Vector3::new(1.0, 0.0, 0.0), 0.5);
        let sphere = MovingSphere::new(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
            0.0,
            1.0,
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        );

        let record = sphere.hit(&ray, 0.0, f32::INFINITY);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(2.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                2.0,
                true,
            )),
        );
    }

    #[test]
    fn hit_sphere_misses_t_min() {
        let ray = Ray::new_at_time(Point3::zero(), Vector3::new(1.0, 0.0, 0.0), 0.5);
        let sphere = MovingSphere::new(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
            0.0,
            1.0,
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        );

        let record = sphere.hit(&ray, 5.0, f32::INFINITY);

        let expected: Option<HitRecord> = None;
        assert_eq!(
            record,
            expected,
        );
    }

    #[test]
    fn hit_sphere_misses_t_max() {
        let ray = Ray::new_at_time(Point3::zero(), Vector3::new(1.0, 0.0, 0.0), 0.5);
        let sphere = MovingSphere::new(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
            0.0,
            1.0,
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        );

        let record = sphere.hit(&ray, 0.0, 1.0);

        let expected: Option<HitRecord> = None;
        assert_eq!(
            record,
            expected,
        );
    }

    #[test]
    fn hit_sphere_other_root() {
        let ray = Ray::new_at_time(Point3::zero(), Vector3::new(1.0, 0.0, 0.0), 0.5);
        let sphere = MovingSphere::new(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(4.0, 0.0, 0.0),
            0.0,
            1.0,
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        );

        let record = sphere.hit(&ray, 3.0, f32::INFINITY);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(4.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                4.0,
                false,
            )),
        );
    }

    #[test]
    fn moving_sphere_bounding_box() {
        let s = MovingSphere::new(
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(3.0, 2.0, 3.0),
            1.0,
            3.0,
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        );

        assert_eq!(
            s.bounding_box(1.0, 2.0),
            Some(BoundingBox::new(
                Point3::new(0.0, 1.0, 2.0),
                Point3::new(3.0, 3.0, 4.0),
            )),
        );
    }
}
