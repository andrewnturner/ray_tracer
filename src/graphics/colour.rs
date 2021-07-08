use std::ops::{Add, Mul};

use num_traits::float::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Colour<T: Float> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Float> Colour<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Colour {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl<T: Float> Add for Colour<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl<T: Float> Mul<T> for Colour<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self { r: self.r * scalar, g: self.g * scalar, b: self.b * scalar }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_eq_delta;

    use super::*;
    
    #[test]
    fn new_colour() {
        assert_eq!(
            Colour::new(0.1, 0.2, 0.3),
            Colour { r: 0.1, g: 0.2, b: 0.3 },
        );
    }

    #[test]
    fn add_colour() {
        let out = Colour::new(0.1, 0.2, 0.3) + Colour::new(0.2, 0.4, 0.6);
        let expected = Colour::new(0.3, 0.6, 0.9);

        assert_eq_delta!(out.r, expected.r, 0.0001);
        assert_eq_delta!(out.g, expected.g, 0.0001);
        assert_eq_delta!(out.b, expected.b, 0.0001);
    }

    #[test]
    fn multiply_scalar() {
        assert_eq!(
            Colour::new(1.0, 2.0, 3.0) * 2.0,
            Colour::new(2.0, 4.0, 6.0),
        );
    }
}
