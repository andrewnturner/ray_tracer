use std::any::Any;

use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;
use crate::util::random::random_in_unit_sphere;

use super::super::hit_record::HitRecord;
use super::super::material::Material;

// Not a true Lambertian.
#[derive(Debug, PartialEq, Clone)]
pub struct Lambertian {
    pub albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Lambertian {
            albedo: albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = hit_record.normal + random_in_unit_sphere();
        
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.albedo,
            Ray::new(hit_record.point, scatter_direction),
        ))
    }

    fn eq(&self, other: &dyn Material) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lambertian() {
        assert_eq!(
            Lambertian::new(Colour::new(0.1, 0.2, 0.3)),
            Lambertian {
                albedo: Colour::new(0.1, 0.2, 0.3),
            },
        );
    }
}