use super::point::Point3;
use super::ray::Ray;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoundingBox {
    pub min: Point3,
    pub max: Point3,
}

impl BoundingBox {
    pub fn new(min: Point3, max: Point3) -> Self {
        if (min.x > max.x) || (min.y > max.y) || (min.z > max.z) {
            panic!("min not less than max");
        }

        BoundingBox {
            min: min,
            max: max,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        // Times of the two hits
        let t_a_x = (self.min.x - ray.origin.x) / ray.direction.x;
        let t_b_x = (self.max.x - ray.origin.x) / ray.direction.x;

        let t_a_y = (self.min.y - ray.origin.y) / ray.direction.y;
        let t_b_y = (self.max.y - ray.origin.y) / ray.direction.y;

        let t_a_z = (self.min.z - ray.origin.z) / ray.direction.z;
        let t_b_z = (self.max.z - ray.origin.z) / ray.direction.z;
        
        // Ordered so t_0 <= t_1
        let t_0_x = t_a_x.min(t_b_x);
        let t_1_x = t_a_x.max(t_b_x);

        let t_0_y = t_a_y.min(t_b_y);
        let t_1_y = t_a_y.max(t_b_y);

        let t_0_z = t_a_z.min(t_b_z);
        let t_1_z = t_a_z.max(t_b_z);

        // Bounded by t_min and t_max
        let t_0b_x = t_0_x.max(t_min);
        let t_1b_x = t_1_x.min(t_max);

        let t_0b_y = t_0_y.max(t_min);
        let t_1b_y = t_1_y.min(t_max);

        let t_0b_z = t_0_z.max(t_min);
        let t_1b_z = t_1_z.min(t_max);

        (t_1b_x > t_0b_x) && (t_1b_y > t_0b_y) && (t_1b_z > t_0b_z)
    }

    pub fn union(&self, other: &Self) -> Self {
        let min = Point3::new(
            self.min.x.min(other.min.x),
            self.min.y.min(other.min.y),
            self.min.z.min(other.min.z),
        );
        let max = Point3::new(
            self.max.x.max(other.max.x),
            self.max.y.max(other.max.y),
            self.max.z.max(other.max.z),
        );

        BoundingBox {
            min: min,
            max: max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::vector::Vector3;

    #[test]
    fn new_bounding_box() {
        assert_eq!(
            BoundingBox::new(
                Point3::new(1.0, 2.0, 3.0),
                Point3::new(4.0, 5.0, 6.0),
            ),
            BoundingBox {
                min: Point3::new(1.0, 2.0, 3.0),
                max: Point3::new(4.0, 5.0, 6.0),
            }
        )
    }

    #[test]
    #[should_panic(expected = "min not less than max")]
    fn new_invalid_bounding_box() {
        BoundingBox::new(
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(0.0, 4.0, 5.0),
        );
    }

    #[test]
    fn hit_bounding_box() {
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        let b = BoundingBox::new(
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(2.0, 1.0, 1.0),
        );

        assert_eq!(
            b.hit(&ray, 1.2, 1.3),
            true,
        );
    }

    #[test]
    fn hit_bounding_box_excludes_t_min() {
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        let b = BoundingBox::new(
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(2.0, 1.0, 1.0),
        );

        assert_eq!(
            b.hit(&ray, 2.5, 3.0),
            false,
        );
    }

    #[test]
    fn hit_bounding_box_excludes_t_max() {
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        );
        let b = BoundingBox::new(
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(2.0, 1.0, 1.0),
        );

        assert_eq!(
            b.hit(&ray, 0.0, 0.5),
            false,
        );
    }

    #[test]
    fn union_bounding_box() {
        let b_1 = BoundingBox::new(
            Point3::new(-3.0, -1.0, 0.0),
            Point3::new(3.0, 1.0, 1.0),
        );

        let b_2 = BoundingBox::new(
            Point3::new(-1.0, -3.0, 0.0),
            Point3::new(1.0, 3.0, 1.0),
        );

        assert_eq!(
            b_1.union(&b_2),
            BoundingBox::new(
                Point3::new(-3.0, -3.0, 0.0),
                Point3::new(3.0, 3.0, 1.0),
            ),
        );
    }
}
