use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Colour {
            r: r,
            g: g,
            b: b,
        }
    }
}

impl Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
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
