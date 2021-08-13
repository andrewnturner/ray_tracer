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
        let (i, u) = integer_and_fractional(p.x);
        let (j, v) = integer_and_fractional(p.y);
        let (k, w) = integer_and_fractional(p.z);

        let u_smooth = u * u * (3.0 - (2.0 * u));
        let v_smooth = v * v * (3.0 - (2.0 * v));
        let w_smooth = w * w * (3.0 - (2.0 * w));

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let r = self.permutation_x[((i + di) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.random_floats[
                        self.permutation_x[((i + di) & 255) as usize] ^
                        self.permutation_y[((j + dj) & 255) as usize] ^
                        self.permutation_z[((k + dk) & 255) as usize]
                    ];
                }
            }
        }

        trilinear_interpolate(c, u_smooth, v_smooth, w_smooth)
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

fn integer_and_fractional(x: f32) -> (isize, f32) {
    (x.floor() as isize, x - x.floor())
}

fn trilinear_interpolate(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                acc += (
                    ((i as f32 * u) + ((1.0 - i as f32) * (1.0 - u))) *
                    ((j as f32 * v) + ((1.0 - j as f32) * (1.0 - v))) *
                    ((k as f32 * w) + ((1.0 - k as f32) * (1.0 - w))) *
                    c[i][j][k]
                )
            }
        }
    }

    acc
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

    #[test]
    fn trilinear_interpolate_top_left() {
        let c = [
            [[1.0, 2.0], [3.0, 4.0]],
            [[5.0, 6.0], [7.0, 8.0]],
        ];

        assert_eq!(
            trilinear_interpolate(c, 0.0, 0.0, 0.0),
            1.0,
        )
    }

    #[test]
    fn trilinear_interpolate_bottom_right() {
        let c = [
            [[1.0, 2.0], [3.0, 4.0]],
            [[5.0, 6.0], [7.0, 8.0]],
        ];

        assert_eq!(
            trilinear_interpolate(c, 1.0, 1.0, 1.0),
            8.0,
        )
    }

    #[test]
    fn trilinear_interpolate_centre() {
        let c = [
            [[1.0, 2.0], [3.0, 4.0]],
            [[5.0, 6.0], [7.0, 8.0]],
        ];

        assert_eq!(
            trilinear_interpolate(c, 0.5, 0.5, 0.5),
            4.5,
        )
    }

    #[test]
    fn integer_and_fractional_negative() {
        assert_eq!(
            integer_and_fractional(-2.5),
            (-3, 0.5),
        );
    }
}
