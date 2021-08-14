use std::any::Any;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;
use crate::util::perlin::Perlin;

use super::super::texture::Texture;

#[derive(PartialEq, Debug)]
pub struct Marble {
    perlin: Perlin,
    scale: f32
}

impl Marble {
    pub fn new(perlin: Perlin, scale: f32) -> Self {
        Marble {
            perlin: perlin,
            scale: scale,
        }
    }
}

impl Texture for Marble {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Colour {
        let weight = ((self.scale * p.z) + (10.0 * self.perlin.turbulence(p, 7))).sin();
        Colour::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + weight)
    }

    fn eq(&self, other: &dyn Texture) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_marble() {
        let _marble = Marble::new(Perlin::new(), 1.0);
    }

    #[test]
    fn marble_value() {
        let marble = Marble::new(Perlin::new(), 1.0);
        let c = marble.value(0.5, 0.5, &Point3::new(1.0, 2.0, 3.0));

        assert!((0.0 <= c.r) && (c.b <= 1.0));
    }
}
