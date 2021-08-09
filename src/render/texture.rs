use std::any::Any;
use std::fmt::Debug;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;

pub trait Texture : Debug {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Colour;

    fn eq(&self, other: &dyn Texture) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<'a, 'b> PartialEq<dyn Texture +  'b> for dyn Texture + 'a {
    fn eq(&self, other:&(dyn Texture + 'b)) -> bool {
        Texture::eq(self, other)
    }
}
