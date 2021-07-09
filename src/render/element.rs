use num_traits::float::Float;

use crate::geometry::ray::Ray;

use super::hit_record::HitRecord;

pub trait Element<T: Float> {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}
