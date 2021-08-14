use std::any::Any;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;
use crate::util::perlin::Perlin;

use super::super::texture::Texture;

#[derive(PartialEq, Debug)]
pub struct Noise {
    perlin: Perlin,
    scale: f32
}

impl Noise {
    pub fn new(perlin: Perlin, scale: f32) -> Self {
        Noise {
            perlin: perlin,
            scale: scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: &Point3) -> Colour {
        Colour::new(1.0, 1.0, 1.0) * self.perlin.turbulence(&(*p * self.scale), 7)
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
    fn new_noise() {
        let _noise = Noise::new(Perlin::new(), 1.0);
    }

    #[test]
    fn noise_value() {
        let noise = Noise::new(Perlin::new(), 1.0);
        let c = noise.value(0.5, 0.5, &Point3::new(1.0, 2.0, 3.0));

        assert!((0.0 <= c.r) && (c.b <= 1.0));
    }
}
