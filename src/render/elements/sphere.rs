use num_traits::cast::FromPrimitive;
use num_traits::float::Float;

use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;

use super::super::element::Element;
use super::super::hit_record::HitRecord;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere<T: Float> {
    pub centre: Point3<T>,
    pub radius: T,
}

impl<T: Float> Sphere<T> {
    pub fn new(centre: Point3<T>, radius: T) -> Self {
        Sphere {
            centre: centre,
            radius: radius,
        }
    }
}

impl<T: Float + FromPrimitive> Element<T> for Sphere<T> {

    /// We have a ray R(t) = A + tb and sphere of radius r centred at C. We have an intersection
    /// if there is t such that
    ///     (R(t) - C) . (R(t) - C) = r^2.
    /// Expanding gives
    ///     (t^2)(b . b) + 2t(b . (A - C)) + ((A - C) . A - C) - r^2 = 0,
    /// a quadratic in t. We can then look at the discriminant ot see whether there are
    /// any solutions.
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let oc = ray.origin - self.centre.clone();
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);

        if discriminant < T::zero() {
            return None
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = ((half_b * T::from_f32(-1.0).unwrap()) - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = ((half_b * T::from_f32(-1.0).unwrap()) + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None
            }
        }

        let p = ray.at(root);
        let normal = (p - self.centre) / self.radius;

        Some(HitRecord::new(p, normal, root))
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector3;

    use super::*;

    #[test]
    fn new_sphere() {
        assert_eq!(
            Sphere::new(
                Point3::new(1.0, 2.0, 3.0),
                5.0,
            ),
            Sphere {
                centre: Point3 { x: 1.0, y: 2.0, z: 3.0 },
                radius: 5.0,
            }
        )
    }

    #[test]
    fn hit_sphere() {
        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Point3::new(3.0, 0.0, 0.0), 1.0);

        let record = sphere.hit(&ray, 0.0, f32::INFINITY);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(2.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                2.0,
            )),
        );
    }

    #[test]
    fn hit_sphere_misses_t_min() {
        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Point3::new(3.0, 0.0, 0.0), 1.0);

        let record = sphere.hit(&ray, 5.0, f32::INFINITY);

        assert_eq!(
            record,
            None,
        );
    }

    #[test]
    fn hit_sphere_misses_t_max() {
        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Point3::new(3.0, 0.0, 0.0), 1.0);

        let record = sphere.hit(&ray, 0.0, 1.0);

        assert_eq!(
            record,
            None,
        );
    }

    #[test]
    fn hit_sphere_other_root() {
        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));
        let sphere = Sphere::new(Point3::new(3.0, 0.0, 0.0), 1.0);

        let record = sphere.hit(&ray, 3.0, f32::INFINITY);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(4.0, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
                4.0,
            )),
        );
    }
}
