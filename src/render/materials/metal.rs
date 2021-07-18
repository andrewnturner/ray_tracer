use std::any::Any;

use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;
use crate::util::random::random_in_unit_sphere;

use super::super::hit_record::HitRecord;
use super::super::material::Material;

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Colour,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Metal {
            albedo: albedo,
            fuzz: 0.0,
        }
    }

    pub fn new_with_fuzz(albedo: Colour, fuzz: f32) -> Self {
        Metal {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let reflected = reflect(ray.direction.normalise(), hit_record.normal);
        let fuzz_vector = random_in_unit_sphere() * self.fuzz;
        let scattered =  Ray::new(hit_record.point, reflected + fuzz_vector);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_metal() {
        assert_eq!(
            Metal::new(Colour::new(0.1, 0.2, 0.3)),
            Metal {
                albedo: Colour::new(0.1, 0.2, 0.3),
                fuzz: 0.0,
            }
        );
    }

    #[test]
    fn new_fuzz_metal() {
        assert_eq!(
            Metal::new_with_fuzz(Colour::new(0.1, 0.2, 0.3), 0.5),
            Metal {
                albedo: Colour::new(0.1, 0.2, 0.3),
                fuzz: 0.5,
            }
        );
    }
}
