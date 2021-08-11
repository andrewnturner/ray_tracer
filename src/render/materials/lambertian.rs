use std::any::Any;
use std::rc::Rc;

use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;
use crate::util::random::random_in_unit_sphere;

use super::super::hit_record::HitRecord;
use super::super::material::Material;
use super::super::texture::Texture;
use super::super::textures::solid_colour::SolidColour;

// Not a true Lambertian.
#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Lambertian {
            albedo: albedo,
        }
    }

    pub fn new_with_colour(albedo: Colour) -> Self {
        Lambertian {
            albedo: Rc::new(SolidColour::new(albedo)),
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
            self.albedo.value(hit_record.u, hit_record.v, &hit_record.point),
            Ray::new_at_time(hit_record.point, scatter_direction, ray.time),
        ))
    }

    fn eq(&self, other: &dyn Material) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Lambertian {
    fn eq(&self, other: &Self) -> bool {
        *self.albedo == *other.albedo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_lambertian_with_colour() {
        assert_eq!(
            Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3)),
            Lambertian {
                albedo: Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
            },
        );
    }
}