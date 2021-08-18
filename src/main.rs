mod geometry;
mod graphics;
mod render;
mod scenes;
mod util;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use num::clamp;
use rand::{Rng, thread_rng};

use geometry::point::Point3;
use geometry::ray::Ray;
use geometry::vector::Vector3;
use graphics::colour::Colour;
use render::camera::Camera;
use render::element::Element;
use scenes::{create_basic_spheres, create_noise_spheres, create_globe, create_lit_globe};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as isize;

    let samples_per_pixel = 40;
    let max_depth = 30;

    let world_choice = 0;
    let (world, look_at, look_from, vfov, background) = match world_choice {
        0 => {
            let world = create_basic_spheres();
            let look_at = Point3::new(0.0, 0.0, -1.0);
            let look_from = Point3::new(-2.0, 2.0, 1.0);
            let vfov = 45.0;
            let background = Colour::new(0.7, 0.8, 1.0);

            (world, look_at, look_from, vfov, background)
        },
        1 => {
            let world = create_noise_spheres();
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let vfov = 20.0;
            let background = Colour::new(0.7, 0.8, 1.0);

            (world, look_at, look_from, vfov, background)
        },
        2 => {
            let world = create_globe();
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let vfov = 20.0;
            let background = Colour::new(0.7, 0.8, 1.0);

            (world, look_at, look_from, vfov, background)
        },
        3 => {
            let world = create_lit_globe();
            let look_at = Point3::new(0.0, 2.0, 0.0);
            let look_from = Point3::new(26.0, 3.0, 6.0);
            let vfov = 20.0;
            let background = Colour::new(0.0, 0.0, 0.0);

            (world, look_at, look_from, vfov, background)
        }
        _ => panic!("Invalid world choice"),
    };    

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        0.1,
        (look_at - look_from).length(),
        0.0,
        1.0,
    );

    let path = Path::new("render.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };

    file.write_all("P3\n".as_bytes()).unwrap();
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes()).unwrap();
    file.write_all("255\n".as_bytes()).unwrap();

    let mut rng = thread_rng();

    // Our coordinates have the origin bottom left.
    for j in (0..image_height).rev() {
        println!("Line {} of {}", j + 1, image_height);

        for i in 0..image_width {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                let u_offset: f32 = rng.gen();
                let v_offset: f32 = rng.gen();

                let u = ((i as f32) + u_offset) / ((image_width - 1) as f32);
                let v = ((j as f32) + v_offset) / ((image_height - 1) as f32);

                let ray = camera.get_ray(u, v);

                pixel_colour += ray_colour(&ray, &background, &world, max_depth);
            }
            
            write_colour(&mut file, &pixel_colour, samples_per_pixel);
        }
    }
}

fn ray_colour(ray: &Ray, background: &Colour, world: &Box<dyn Element>, depth: isize) -> Colour {
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    
    match world.hit(&ray, 0.001, f32::INFINITY) {
        Some(hit_record) => {
            let emitted = hit_record.material.emit(hit_record.u, hit_record.v, &hit_record.point);
            match hit_record.material.scatter(&ray, &hit_record) {
                Some((attenuation, scattered)) => {
                    emitted + (ray_colour(&scattered, background, world, depth - 1) * attenuation)
                },
                None => emitted,
            }
        },
        None => *background,
    }
}

fn write_colour(file: &mut File, colour: &Colour, samples_per_pixel: isize) {
    let scale = 1.0 / (samples_per_pixel as f32);

    // Gamma correction with gamma = 2.0
    let sr = (scale * colour.r).sqrt();
    let sg = (scale * colour.g).sqrt();
    let sb = (scale * colour.b).sqrt();

    let ir = (256.0 * clamp(sr, 0.0, 0.999)) as isize;
    let ig = (256.0 * clamp(sg, 0.0, 0.999)) as isize;
    let ib = (256.0 * clamp(sb, 0.0, 0.999)) as isize;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
}
