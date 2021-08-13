use rand::{Rng, thread_rng};

use crate::geometry::point::Point3;

const POINT_COUNT: usize = 256;

#[derive(Debug, PartialEq)]
pub struct Perlin {
    random_floats: [f32; POINT_COUNT],
    permutation_x: [usize; POINT_COUNT],
    permutation_y: [usize; POINT_COUNT],
    permutation_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let mut random_floats = [0.0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            random_floats[i] = rng.gen();
        }

        Perlin {
            random_floats: random_floats,
            permutation_x: Self::make_permutation(),
            permutation_y: Self::make_permutation(),
            permutation_z: Self::make_permutation(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let i = (((4.0 * p.x) as isize) & 255) as usize;
        let j = (((4.0 * p.y) as isize) & 255) as usize;
        let k = (((4.0 * p.z) as isize) & 255) as usize;

        let c = self.permutation_x[i] ^ self.permutation_y[j] ^ self.permutation_z[k];

        self.random_floats[c]
    }

    fn make_permutation() -> [usize; POINT_COUNT] {
        let mut rng = thread_rng();
        
        let mut p = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i;
        }

        for i in (1..POINT_COUNT).rev() {
            let target = rng.gen_range(0..i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }

        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_perlin() {
        let _perlin = Perlin::new();
    }

    #[test]
    fn perlin_noise() {
        let perlin = Perlin::new();
        let x = perlin.noise(&Point3::new(1.0, 2.0, 3.0));

        assert!((0.0 <= x) && (x <= 1.0));
    }
}
