use std::any::Any;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;

use super::super::texture::Texture;

#[derive(Debug, PartialEq)]
pub struct SolidColour {
    pub colour_value: Colour,
}

impl SolidColour {
    pub fn new(colour_value: Colour) -> Self {
        SolidColour {
            colour_value: colour_value,
        }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f32, _v: f32, _p: &Point3) -> Colour {
        self.colour_value
    }

    fn eq(&self, other: &dyn Texture) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn new_solid_colour() {
        assert_eq!(
            SolidColour::new(Colour::new(0.1, 0.2, 0.3)),
            SolidColour { colour_value: Colour::new(0.1, 0.2, 0.3) }
        );
    }

    #[test]
    fn solid_colour_value() {
        let texture = SolidColour::new(Colour::new(0.1, 0.2, 0.3));

        assert_eq!(
            texture.value(0.2, 0.4, &Point3::new(1.0, 2.0, 3.0)),
            Colour::new(0.1, 0.2, 0.3),
        );
    }
}
