use rand::{Rng, thread_rng};

use crate::geometry::point::Point3;
use crate::geometry::ray::Ray;
use crate::geometry::vector::Vector3;
use crate::util::random::random_in_unit_disk;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vector3,
    vertical: Vector3,
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vector3,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
        time_0: f32,
        time_1: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalise();
        let u = v_up.cross(&w).normalise();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_distance);

        let lens_radius = aperture / 2.0;

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
            time_0: time_0,
            time_1: time_1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let point_on_lens = random_in_unit_disk() * self.lens_radius;
        let offset = (self.u * point_on_lens.x) + (self.v * point_on_lens.y);

        let mut rng = thread_rng();

        let ray = Ray::new_at_time(
            self.origin + offset,
            self.lower_left_corner + (self.horizontal * s) + (self.vertical * t) - (self.origin + offset),
            rng.gen_range(self.time_0..self.time_1),
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
                4.0 / 3.0,
                0.5,
                2.0,
                0.0,
                1.0,
            ).origin,
            Point3::new(0.0, 0.0, 0.0),
        );
    }
}
