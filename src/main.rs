mod geometry;
mod graphics;
mod render;
mod util;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use num_traits::cast::FromPrimitive;
use num_traits::float::Float;

use geometry::point::Point3;
use geometry::ray::Ray;
use geometry::vector::Vector3;
use render::element::Element;
use render::elements::sphere::Sphere;
use graphics::colour::Colour;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as isize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let depth = Vector3::new(0.0, 0.0, focal_length);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - depth;

    let path = Path::new("render.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };

    file.write_all("P3\n".as_bytes());
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes());
    file.write_all("255\n".as_bytes());

    for j in 0..image_height {
        println!("Line {} of {}", j + 1, image_height);

        for i in 0..image_width {
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);

            let r = Ray::new(
                origin,
                (lower_left_corner + (horizontal * u) + (vertical * v)).as_vector3(),
            );

            let pixel_colour = ray_colour(&r);
            write_colour(&mut file, &pixel_colour);
        }
    }
}

fn ray_colour<T: Float + FromPrimitive>(ray: &Ray<T>) -> Colour<T> {
    let white = Colour::new(
        T::from_f32(1.0).unwrap(),
        T::from_f32(1.0).unwrap(),
        T::from_f32(1.0).unwrap(),
    );
    let blue = Colour::new(
        T::from_f32(0.5).unwrap(),
        T::from_f32(0.7).unwrap(),
        T::from_f32(1.0).unwrap(),
    );
    let red = Colour::new(
        T::from_f32(1.0).unwrap(),
        T::from_f32(0.0).unwrap(),
        T::from_f32(0.0).unwrap(),
    );
    
    let sphere = Sphere::new(
        Point3::new(
            T::from_f32(0.0).unwrap(),
            T::from_f32(0.0).unwrap(),
            T::from_f32(-1.0).unwrap(),
        ),
        T::from_f32(0.5).unwrap(),
    );
    if let Some(hit_record) = sphere.hit(&ray, T::zero(), T::infinity()) {
        let normal = hit_record.normal.normalise();
        return Colour::new(
            normal.x + T::from_f32(1.0).unwrap(),
            normal.y + T::from_f32(1.0).unwrap(),
            normal.z + T::from_f32(1.0).unwrap(),
        ) * T::from_f32(0.5).unwrap();
    }
    
    let unit = ray.direction.normalise();
    let t: T = T::from_f32(0.5).unwrap() * (unit.y + T::from_f32(1.0).unwrap());

    (white * (T::from_f32(1.0).unwrap() - t)) + (blue * t)
}

fn write_colour<T: Float + FromPrimitive>(file: &mut File, colour: &Colour<T>) {
    let scale: T = T::from_f32(255.999).unwrap();

    let sr: T = scale * colour.r;
    let sg: T = scale * colour.g;
    let sb: T = scale * colour.b;

    let ir: isize = sr.to_isize().unwrap();
    let ig: isize = sg.to_isize().unwrap();
    let ib: isize = sb.to_isize().unwrap();

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes());
}
