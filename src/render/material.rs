use std::any::Any;
use std::fmt::Debug;

use crate::geometry::ray::Ray;
use crate::graphics::colour::Colour;

use super::hit_record::HitRecord;

pub trait Material : Debug {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Colour, Ray)>;

    fn eq(&self, other: &dyn Material) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<'a, 'b> PartialEq<dyn Material +  'b> for dyn Material + 'a {
    fn eq(&self, other:&(dyn Material + 'b)) -> bool {
        Material::eq(self, other)
    }
}
