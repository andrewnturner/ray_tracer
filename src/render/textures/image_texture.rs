use std::any::Any;

use image::RgbImage;
use image::io::Reader as ImageReader;
use num::clamp;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;

use super::super::texture::Texture;

#[derive(Debug, PartialEq)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(image: RgbImage) -> Self {
        ImageTexture {
            image: image,
        }
    }

    pub fn new_from_filename(filename: &str) -> Self {
        let image = ImageReader::open(filename).unwrap().decode().unwrap();

        ImageTexture {
            image: image.into_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3) -> Colour {
        let (width, height) = self.image.dimensions();
        let i = ((clamp(u, 0.0, 1.0) * width as f32) as u32).min(width);
        let j = ((clamp(v, 0.0, 1.0) * height as f32) as u32).min(height);

        // Origin is bottom left in our coordinates, but the image coordinates
        // have it top left. So need to flip vertically.
        let pixel = self.image.get_pixel(i, height - j);

        let scale = 255.0_f32.recip();

        Colour::new(
            scale * pixel[0] as f32,
            scale * pixel[1] as f32,
            scale * pixel[2] as f32,
        )
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
    use image::Rgb;

    use super::*;

    #[test]
    fn new_image_texture() {
        assert_eq!(
            ImageTexture::new(RgbImage::new(10, 10)),
            ImageTexture {
                image: RgbImage::new(10, 10),
            },
        );
    }

    #[test]
    fn image_texture_value() {
        let mut image = RgbImage::new(10, 10);
        image.put_pixel(5, 5, Rgb([255, 0, 0]));

        let image_texture = ImageTexture::new(image);

        assert_eq!(
            image_texture.value(0.5, 0.5, &Point3::new(0.0, 0.0, 0.0)),
            Colour::new(1.0, 0.0, 0.0),
        );
    }
}
