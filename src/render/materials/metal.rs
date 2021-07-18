use std::any::Any;

use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;

use super::super::hit_record::HitRecord;
use super::super::material::Material;

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Metal {
            albedo: albedo,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.direction.normalise(), hit_record.normal);
        let scattered =  Ray::new(hit_record.point, reflected);

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((
                self.albedo,
                scattered,
            ))
        } else {
            None
        }
    }

    fn eq(&self, other: &dyn Material) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v + (n * (-2.0 * v.dot(&n)))
}
