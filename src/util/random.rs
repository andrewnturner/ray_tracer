use rand::{Rng, thread_rng};

use crate::geometry::vector::Vector3;

pub fn random_in_unit_sphere() -> Vector3 {
    let mut rng = thread_rng();

    loop {
        let p = random_in_unit_cube();
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3 {
    let mut rng = thread_rng();

    loop {
        let p = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_cube() -> Vector3 {
    let mut rng = thread_rng();

    Vector3::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_in_unit_sphere() {
        let p = random_in_unit_sphere();

        assert!(p.length_squared() <= 1.0);
    }

    #[test]
    fn test_random_in_unit_disk() {
        let p = random_in_unit_disk();

        assert!(p.length_squared() <= 1.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn test_random_in_unit_cube() {
        let p = random_in_unit_cube();

        assert!((-1.0 <= p.x) && (p.x <= 1.0));
        assert!((-1.0 <= p.y) && (p.y <= 1.0));
        assert!((-1.0 <= p.z) && (p.z <= 1.0));
    }
}