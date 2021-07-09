use std::fmt::Debug;

use crate::geometry::ray::Ray;

use super::hit_record::HitRecord;

pub trait Element {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
