use std::any::Any;
use std::cmp::Ordering;
use std::rc::Rc;

use rand::{Rng, thread_rng};

use crate::geometry::bounding_box::BoundingBox;
use crate::geometry::ray::Ray;
use crate::render::hit_record::HitRecord;

use super::super::element::Element;

#[derive(Debug)]
pub struct BvhNode {
    left: Rc<dyn Element>,
    right: Rc<dyn Element>,
    bounding_box: BoundingBox,
}

impl BvhNode {
    pub fn new(left: Rc<dyn Element>, right: Rc<dyn Element>, bounding_box: BoundingBox) -> Self {
        BvhNode {
            left: left,
            right: right,
            bounding_box: bounding_box,
        }
    }

    pub fn from_elements(mut elements: Vec<Rc<dyn Element>>, time_0: f32, time_1: f32) -> Self {
        let length = elements.len();

        Self::from_elements_inner(
            &mut elements,
            0,
            length,
            time_0,
            time_1,
        )
    }

    fn from_elements_inner(
        elements: &mut Vec<Rc<dyn Element>>,
        start: usize,
        end: usize,
        time_0: f32,
        time_1: f32,
    ) -> Self {
        let mut rng = thread_rng();
        let comparator = match rng.gen_range(0..3) {
            0 => compare_bounding_box_x,
            1 => compare_bounding_box_y,
            2 => compare_bounding_box_z,
            _ => panic!("Should be one of 0, 1, 2"),
        };

        let span: usize = end - start;
        let nodes = match span {
            1 => (elements[start].clone(), elements[start].clone()),
            2 => {
                match comparator(&elements[start], &elements[start + 1]) {
                    Ordering::Less | Ordering::Equal => (elements[start].clone(), elements[start + 1].clone()),
                    Ordering::Greater => (elements[start + 1].clone(), elements[start].clone()),
                }
            },
            _ => {
                elements[start..end].sort_unstable_by(comparator);

                let mid = start + (span / 2);
                let left_node: Rc<dyn Element> = Rc::new(
                    Self::from_elements_inner(elements, start, mid, time_0, time_1)
                );
                let right_node: Rc<dyn Element> = Rc::new(
                    Self::from_elements_inner(elements, mid, end, time_0, time_1)
                );

                (left_node, right_node)
            }
        };

        let box_left = nodes.0.bounding_box(time_0, time_1).unwrap();
        let box_right = nodes.1.bounding_box(time_0, time_1).unwrap();

        Self::new(
            nodes.0,
            nodes.1,
            box_left.union(&box_right),
        )
    }
}

impl Element for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let t_max_right = match hit_left {
            Some(ref hit_record) => hit_record.t,
            None => t_max,
        };
        let hit_right = self.right.hit(ray, t_min, t_max_right);

        match hit_right {
            Some(ref _hit_record) => hit_right,
            None => hit_left,
        }
    }

    fn bounding_box(&self, _time_0: f32, _time_1: f32) -> Option<BoundingBox> {
        Some(self.bounding_box)
    }

    fn eq(&self, other: &dyn Element) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for BvhNode {
    fn eq(&self, other: &Self) -> bool {
        (*self.left == *other.left) &&
        (*self.right == *other.right) &&
        (self.bounding_box == other.bounding_box)
    }
}

fn compare_bounding_box_x(a: &Rc<dyn Element>, b: &Rc<dyn Element>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a.min.x.partial_cmp(&box_b.min.x).unwrap()
}

fn compare_bounding_box_y(a: &Rc<dyn Element>, b: &Rc<dyn Element>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a.min.y.partial_cmp(&box_b.min.y).unwrap()
}

fn compare_bounding_box_z(a: &Rc<dyn Element>, b: &Rc<dyn Element>) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();

    box_a.min.z.partial_cmp(&box_b.min.z).unwrap()
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::geometry::point::Point3;
    use crate::geometry::vector::Vector3;
    use crate::graphics::colour::Colour;
    use crate::render::materials::lambertian::Lambertian;

    use super::*;
    use super::super::sphere::Sphere;

    #[test]
    pub fn new_bvh_node() {
        let sphere_1 = Rc::new(Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let sphere_2 = Rc::new(Sphere::new(
            Point3::new(1.0, 2.0, 3.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let bounding_box = BoundingBox::new(
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(2.0, 3.0, 4.0),
        );

        assert_eq!(
            BvhNode::new(sphere_1, sphere_2, bounding_box),
            BvhNode {
                left: Rc::new(Sphere::new(
                    Point3::new(1.0, 2.0, 3.0),
                    1.0,
                    Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                )),
                right: Rc::new(Sphere::new(
                    Point3::new(1.0, 2.0, 3.0),
                    1.0,
                    Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                )),
                bounding_box: bounding_box,
            },
        );
    }

    #[test]
    fn hit_bvh_node_left() {
        let sphere_1 = Rc::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let sphere_2 = Rc::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let bounding_box = BoundingBox::new(
            Point3::new(3.0, -1.0, -1.0),
            Point3::new(8.0, 1.0, 1.0),
        );
        let bvh_node = BvhNode::new(sphere_1, sphere_2, bounding_box);

        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );

        let hit_record = bvh_node.hit(&ray, 0.0, 10.0);

        assert_eq!(
            hit_record,
            Some(HitRecord::new(
                Point3::new(3.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                3.0,
                true
            )),
        );
    }

    #[test]
    fn hit_bvh_node_right() {
        let sphere_1 = Rc::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let sphere_2 = Rc::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let bounding_box = BoundingBox::new(
            Point3::new(3.0, -1.0, -1.0),
            Point3::new(8.0, 1.0, 1.0),
        );
        let bvh_node = BvhNode::new(sphere_1, sphere_2, bounding_box);

        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );

        let hit_record = bvh_node.hit(&ray, 0.0, 10.0);

        assert_eq!(
            hit_record,
            Some(HitRecord::new(
                Point3::new(3.0, 0.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
                3.0,
                true
            )),
        );
    }

    #[test]
    fn ray_misses_bounding_box() {
        let sphere_1 = Rc::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let sphere_2 = Rc::new(Sphere::new(
            Point3::new(7.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        ));
        let bounding_box = BoundingBox::new(
            Point3::new(3.0, -1.0, -1.0),
            Point3::new(8.0, 1.0, 1.0),
        );
        let bvh_node = BvhNode::new(sphere_1, sphere_2, bounding_box);

        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );

        let hit_record = bvh_node.hit(&ray, 0.0, 10.0);

        assert_eq!(
            hit_record,
            None,
        );
    }

    #[test]
    fn bvh_node_from_single_element() {
        let mut elements: Vec<Rc<dyn Element>> = Vec::new();
        elements.push(Rc::new(Sphere::new(
            Point3::new(4.0, 0.0, 0.0),
            1.0,
            Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
        )));

        let bvh_node = BvhNode::from_elements(elements, 0.0, 1.0);

        let expected = BvhNode::new(
            Rc::new(Sphere::new(
                Point3::new(4.0, 0.0, 0.0),
                1.0,
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
            )),
            Rc::new(Sphere::new(
                Point3::new(4.0, 0.0, 0.0),
                1.0,
                Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.3))),
            )),
            BoundingBox::new(
                Point3::new(3.0, -1.0, -1.0),
                Point3::new(5.0, 1.0, 1.0),
            ),
        );

        assert_eq!(
            bvh_node,
            expected,
        );
    }
}
