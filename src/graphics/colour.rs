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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn new_colour() {
        assert_eq!(
            Colour::new(0.1, 0.2, 0.3),
            Colour { r: 0.1, g: 0.2, b: 0.3 },
        );
    }
}
