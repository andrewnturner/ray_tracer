use std::any::Any;

use rand::{thread_rng, Rng};

use crate::geometry::vector::Vector3;
use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;
use crate::render::hit_record::HitRecord;

use super::super::material::Material;
use super::super::materials::metal::reflect;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction: index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.normalise();

        let cos_theta = (unit_direction * -1.0).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let mut rng = thread_rng();

        let direction = if cannot_refract || (reflectance(cos_theta, refraction_ratio) > rng.gen()) {
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let scattered = Ray::new_at_time(hit_record.point, direction, ray.time);

        Some((attenuation, scattered))
    }

    fn eq(&self, other: &dyn Material) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn refract(v: Vector3, n: Vector3, eta_i_over_eta_t: f32) -> Vector3 {
    let cos_theta = (v * -1.0).dot(&n).min(1.0);
    let r_perpendicular = (v + (n * cos_theta)) * eta_i_over_eta_t;
    let r_parallel = n * -(1.0 - r_perpendicular.length_squared()).abs().sqrt();

    r_perpendicular + r_parallel
}

/// Schlick approximation
fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    let sqrt_r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = sqrt_r0 * sqrt_r0;

    r0 + ((1.0 - r0) * (1.0 - cos_theta).powi(5))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dielectric() {
        assert_eq!(
            Dielectric::new(1.0),
            Dielectric { index_of_refraction: 1.0 },
        );
    }
}