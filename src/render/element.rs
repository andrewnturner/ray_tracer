use std::any::Any;
use std::fmt::Debug;

use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;

use super::hit_record::HitRecord;

pub trait Element : Debug {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<BoundingBox>;

    fn eq(&self, other: &dyn Element) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<'a, 'b> PartialEq<dyn Element +  'b> for dyn Element + 'a {
    fn eq(&self, other:&(dyn Element + 'b)) -> bool {
        Element::eq(self, other)
    }
}
