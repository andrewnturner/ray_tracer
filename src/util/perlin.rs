use rand::{Rng, thread_rng};

use arrayvec::ArrayVec;

use crate::geometry::point::Point3;
use crate::geometry::vector::Vector3;
use crate::util::random::random_in_unit_cube;

const POINT_COUNT: usize = 256;

#[derive(Debug, PartialEq)]
pub struct Perlin {
    random_vectors: ArrayVec<Vector3, POINT_COUNT>,
    permutation_x: [usize; POINT_COUNT],
    permutation_y: [usize; POINT_COUNT],
    permutation_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vectors = ArrayVec::<Vector3, POINT_COUNT>::new();
        for _i in 0..POINT_COUNT {
            random_vectors.push(random_in_unit_cube());
        }

        Perlin {
            random_vectors: random_vectors,
            permutation_x: Self::make_permutation(),
            permutation_y: Self::make_permutation(),
            permutation_z: Self::make_permutation(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let (i, u) = integer_and_fractional(p.x);
        let (j, v) = integer_and_fractional(p.y);
        let (k, w) = integer_and_fractional(p.z);

        let mut c = ArrayVec::<ArrayVec<ArrayVec<Vector3, 2>, 2>, 2>::new();
        for di in 0..2 {
            let mut y_array = ArrayVec::new();
            for dj in 0..2 {
                let mut z_array = ArrayVec::new();
                for dk in 0..2 {
                    z_array.push(self.random_vectors[
                        self.permutation_x[((i + di) & 255) as usize] ^
                        self.permutation_y[((j + dj) & 255) as usize] ^
                        self.permutation_z[((k + dk) & 255) as usize]
                    ]);
                }
                y_array.push(z_array);
            }
            c.push(y_array);
        }

        perlin_interpolate(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: usize) -> f32 {
        let mut acc = 0.0;
        let mut weighted_p = *p;
        let mut weight = 1.0;
        for _i in 0..depth {
            acc += weight * self.noise(&weighted_p);
            weight *= 0.5;
            weighted_p = weighted_p * 2.0;
        }

        return acc.abs()
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

// fn trilinear_interpolate(c: ArrayVec<ArrayVec<ArrayVec<Vector3, 2>, 2>, 2>, u: f32, v: f32, w: f32) -> f32 {
//     let mut acc = 0.0;
//     for i in 0..2 {
//         for j in 0..2 {
//             for k in 0..2 {
//                 acc += ((i as f32 * u) + ((1.0 - i as f32) * (1.0 - u))) *
//                        ((j as f32 * v) + ((1.0 - j as f32) * (1.0 - v))) *
//                        ((k as f32 * w) + ((1.0 - k as f32) * (1.0 - w))) *
//                        c[i][j][k];
//             }
//         }
//     }

//     acc
// }

fn perlin_interpolate(c: ArrayVec<ArrayVec<ArrayVec<Vector3, 2>, 2>, 2>, u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - (2.0 * u));
    let vv = v * v * (3.0 - (2.0 * v));
    let ww = w * w * (3.0 - (2.0 * w));

    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight = Vector3::new(u - i as f32, v - j as f32, w - k as f32);
                acc += ((i as f32 * uu) + ((1.0 - i as f32) * (1.0 - uu))) *
                       ((j as f32 * vv) + ((1.0 - j as f32) * (1.0 - vv))) *
                       ((k as f32 * ww) + ((1.0 - k as f32) * (1.0 - ww))) *
                       c[i][j][k].dot(&weight);
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

    // #[test]
    // fn trilinear_interpolate_top_left() {
    //     let c = [
    //         [[1.0, 2.0], [3.0, 4.0]],
    //         [[5.0, 6.0], [7.0, 8.0]],
    //     ];

    //     assert_eq!(
    //         trilinear_interpolate(c, 0.0, 0.0, 0.0),
    //         1.0,
    //     )
    // }

    // #[test]
    // fn trilinear_interpolate_bottom_right() {
    //     let c = [
    //         [[1.0, 2.0], [3.0, 4.0]],
    //         [[5.0, 6.0], [7.0, 8.0]],
    //     ];

    //     assert_eq!(
    //         trilinear_interpolate(c, 1.0, 1.0, 1.0),
    //         8.0,
    //     )
    // }

    // #[test]
    // fn trilinear_interpolate_centre() {
    //     let c = [
    //         [[1.0, 2.0], [3.0, 4.0]],
    //         [[5.0, 6.0], [7.0, 8.0]],
    //     ];

    //     assert_eq!(
    //         trilinear_interpolate(c, 0.5, 0.5, 0.5),
    //         4.5,
    //     )
    // }

    #[test]
    fn integer_and_fractional_negative() {
        assert_eq!(
            integer_and_fractional(-2.5),
            (-3, 0.5),
        );
    }
}
