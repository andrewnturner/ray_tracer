use std::any::Any;
use std::fmt::Debug;

use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;

use super::super::element::Element;
use super::super::hit_record::HitRecord;

#[derive(Debug, PartialEq)]
pub struct ElementList {
    pub elements: Vec<Box<dyn Element>>,
}

impl ElementList {
    pub fn new() -> Self {
        ElementList {
            elements: Vec::new(),
        }
    }

    pub fn add(&mut self, element: Box<dyn Element>) {
        self.elements.push(element);
    }
}

impl Element for ElementList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_t = t_max;
        let mut closest_hit_record = None;

        for element in self.elements.iter() {
            match element.hit(ray, t_min, closest_t) {
                Some(hit_record) => {
                    closest_hit_record = Some(hit_record.clone());
                    closest_t = hit_record.t;
                },
                None => {},
            }
        }

        closest_hit_record
    }

    fn bounding_box(&self, time_0: f32, time_1: f32) -> Option<BoundingBox> {
        let mut surrounding_box: Option<BoundingBox> = None;
        for element in self.elements.iter() {
            if let Some(b) = element.bounding_box(time_0, time_1) {
                surrounding_box = match surrounding_box {
                    Some(a) => Some(a.union(&b)),
                    None => Some(b),
                }
            } else {
                return None;
            }
        }

        surrounding_box
    }

    fn eq(&self, other: &dyn Element) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::geometry::point::Point3;
    use crate::geometry::vector::Vector3;
    use crate::graphics::colour::Colour;
    use crate::render::elements::sphere::Sphere;
    use crate::render::materials::lambertian::Lambertian;

    use super::*;

    #[test]
    fn new_element_list() {
        assert_eq!(
            ElementList::new(),
            ElementList { elements: Vec::new() },
        )
    }

    #[test]
    fn add_to_element_list() {
        let mut list = ElementList::new();
        list.add(Box::new(Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            5.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));

        let mut expected_elements: Vec<Box<dyn Element>> = Vec::new();
        expected_elements.push(
            Box::new(Sphere::new(
                Point3::new(1.0, 2.0, 3.0),
                5.0,
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
            )) as Box<dyn Element>
        );
        let expected = ElementList {
            elements: expected_elements,
        };

        assert_eq!(
            list,
            expected,
        )
    }

    #[test]
    fn hit_correct_element_first() {
        let mut list = ElementList::new();
        list.add(Box::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));
        list.add(Box::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));

        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));

        let record = list.hit(&ray, 0.0, 10.0);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(3.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                3.0,
                0.0,
                0.5,
                true
            )),
        )
    }

    #[test]
    fn hit_correct_element_second() {
        let mut list = ElementList::new();
        list.add(Box::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));
        list.add(Box::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));

        let ray = Ray::new(Point3::zero(), Vector3::new(1.0, 0.0, 0.0));

        let record = list.hit(&ray, 0.0, 10.0);

        assert_eq!(
            record,
            Some(HitRecord::new(
                Point3::new(3.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
                3.0,
                0.0,
                0.5,
                true
            )),
        )
    }

    #[test]
    fn element_list_bounding_box() {
        let mut list = ElementList::new();
        list.add(Box::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));
        list.add(Box::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new_with_colour(Colour::new(0.1, 0.2, 0.3))),
        )));

        assert_eq!(
            list.bounding_box(0.0, 1.0),
            Some(BoundingBox::new(
                Point3::new(3.0, -1.0, -1.0),
                Point3::new(8.0, 1.0, 1.0),
            )),
        );
    }

    #[test]
    fn empty_element_list_bounding_box() {
        let list = ElementList::new();

        assert_eq!(
            list.bounding_box(0.0, 1.0),
            None,
        );
    }
}
