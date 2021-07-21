use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(vertical_fov: f32, aspect_ratio: f32) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point3::zero();
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        let depth = Vector3::new(0.0, 0.0, focal_length);
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - depth;

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            (self.lower_left_corner + (self.horizontal * u) + (self.vertical * v)).as_vector3(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_camera() {
        assert_eq!(
            Camera::new(90.0, 4.0 / 3.0).origin,
            Point3::new(0.0, 0.0, 0.0),
        );
    }
}
