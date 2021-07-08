mod geometry;
mod graphics;
mod util;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use num_traits::float::Float;
use num_traits::cast::FromPrimitive;

use geometry::point::Point3;
use geometry::ray::Ray;
use geometry::vector::Vector3;
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
    
    let sphere_centre = Point3::new(
        T::from_f32(0.0).unwrap(),
        T::from_f32(0.0).unwrap(),
        T::from_f32(-1.0).unwrap(),
    );
    if let Some(t) = hit_sphere(&sphere_centre, T::from_f32(0.5).unwrap(), ray) {
        if t > T::zero() {
            let normal = (ray.at(t) - sphere_centre).normalise();
            return Colour::new(
                normal.x + T::from_f32(1.0).unwrap(),
                normal.y + T::from_f32(1.0).unwrap(),
                normal.z + T::from_f32(1.0).unwrap(),
            ) * T::from_f32(0.5).unwrap();
        }
    }
    
    let unit = ray.direction.normalise();
    let t: T = T::from_f32(0.5).unwrap() * (unit.y + T::from_f32(1.0).unwrap());

    (white * (T::from_f32(1.0).unwrap() - t)) + (blue * t)
}


/// We have a ray R(t) = A + tb and sphere of radius r centred at C. We have an intersection
/// if there is t such that
///     (R(t) - C) . (R(t) - C) = r^2.
/// Expanding gives
///     (t^2)(b . b) + 2t(b . (A - C)) + ((A - C) . A - C) - r^2 = 0,
/// a quadratic in t. We can then look at the discriminant ot see whether there are
/// any solutions.
fn hit_sphere<T: Float + FromPrimitive>(centre: &Point3<T>, radius: T, ray: &Ray<T>) -> Option<T> {
    let oc = ray.origin - centre.clone();
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - (radius * radius);

    let discriminant = (half_b * half_b) - (a * c);

    if discriminant > T::zero() {
        let t = ((half_b * T::from_f32(-1.0).unwrap()) - discriminant.sqrt()) / a;
        Some(t)
    } else {
        None
    }
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
