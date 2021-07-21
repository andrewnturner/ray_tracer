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
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vector3,
        vertical_fov: f32,
        aspect_ratio: f32
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalise();
        let u = v_up.cross(&w).normalise();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
         let ray = Ray::new(
            self.origin,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - self.origin,
        );

        ray
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_camera() {
        assert_eq!(
            Camera::new(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                90.0,
                4.0 / 3.0
            ).origin,
            Point3::new(0.0, 0.0, 0.0),
        );
    }
}
