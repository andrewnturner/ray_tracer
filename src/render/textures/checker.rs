use std::any::Any;
use std::rc::Rc;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;

use super::super::texture::Texture;

#[derive(Debug)]
pub struct Checker {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl Checker {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Checker {
            odd: odd,
            even: even,
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Colour {
        let sines = (p.x * 20.0).sin() * (p.y * 20.0).sin() * (p.z * 20.0).sin();
        let sines2 = (u * 50.0).sin() * (v * 50.0).sin();
        if sines2 < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }

    fn eq(&self, other: &dyn Texture) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PartialEq for Checker {
    fn eq(&self, other: &Self) -> bool {
        (*self.odd == *other.odd) && (*self.even == *other.even)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::solid_colour::SolidColour;

    #[test]
    fn new_checker() {
        assert_eq!(
            Checker::new(
                Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
                Rc::new(SolidColour::new(Colour::new(0.4, 0.5, 0.6))),
            ),
            Checker {
                odd: Rc::new(SolidColour::new(Colour::new(0.1, 0.2, 0.3))),
                even: Rc::new(SolidColour::new(Colour::new(0.4, 0.5, 0.6))),
            }
        );
    }
}
