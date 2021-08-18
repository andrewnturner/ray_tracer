use std::any::Any;
use std::rc::Rc;

use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;

use super::super::hit_record::HitRecord;
use super::super::material::Material;
use super::super::texture::Texture;

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        DiffuseLight {
            emit: emit,
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }

    fn emit(&self, u: f32, v: f32, p: &Point3) -> Colour {
        self.emit.value(u, v, p)
    }

    fn eq(&self, other: &dyn Material) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for DiffuseLight {
    fn eq(&self, other: &Self) -> bool {
        *self.emit == *other.emit
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::vector::Vector3;
    use crate::render::materials::lambertian::Lambertian;
    use crate::render::textures::solid_colour::SolidColour;

    use super::*;

    #[test]
    fn new_diffuse_light() {
        assert_eq!(
            DiffuseLight::new(
                Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
            ),
            DiffuseLight {
                emit: Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
            },
        );
    }

    #[test]
    fn doesnt_scatter() {
        let light = DiffuseLight::new(
            Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
        );
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        let hit_record = HitRecord::new(
            Point3::new(3.0, 0.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
            3.0,
            0.0,
            0.5,
            true
        );

        assert_eq!(
            light.scatter(&ray, &hit_record),
            None,
        );
    }

    #[test]
    fn diffuse_light_emit() {
        let light = DiffuseLight::new(
            Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
        );

        assert_eq!(
            light.emit(0.5, 0.5, &Point3::new(1.0, 2.0, 3.0)),
            Colour::new(0.1, 0.2, 0.3),
        );
    }
}
