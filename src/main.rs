mod geometry;
mod graphics;
mod render;
mod util;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

use num::clamp;
use rand::{Rng, thread_rng};

use geometry::point::Point3;
use geometry::ray::Ray;
use geometry::vector::Vector3;
use graphics::colour::Colour;
use render::camera::Camera;
use render::element::Element;
use render::elements::bvh_node::BvhNode;
use render::elements::moving_sphere::MovingSphere;
use render::elements::sphere::Sphere;
use render::materials::dielectric::Dielectric;
use render::materials::lambertian::Lambertian;
use render::materials::metal::Metal;
use render::textures::checker::Checker;
use render::textures::noise::Noise;
use render::textures::solid_colour::SolidColour;
use util::perlin::Perlin;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as isize;

    let samples_per_pixel = 20;
    let max_depth = 20;

    let world_choice = 1;
    let (world, look_at, look_from, vfov) = match world_choice {
        0 => {
            let world = create_world0();
            let look_at = Point3::new(-2.0, 2.0, 1.0);
            let look_from = Point3::new(0.0, 0.0, -1.0);
            let vfov = 45.0;

            (world, look_at, look_from, vfov)
        },
        1 => {
            let world = create_world1();
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let vfov = 20.0;

            (world, look_at, look_from, vfov)
        },
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

                pixel_colour += ray_colour(&ray, &world, max_depth);
            }
            
            write_colour(&mut file, &pixel_colour, samples_per_pixel);
        }
    }
}

fn create_world0() -> Box<dyn Element> {
    let material_ground = Rc::new(Lambertian::new_with_colour(Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Rc::new(
        Lambertian::new(
            Rc::new(Checker::new(
                Rc::new(SolidColour::new(Colour::new(0.1, 0.1, 0.1))),
                Rc::new(SolidColour::new(Colour::new(0.7, 0.7, 0.7))),
            )),
        ),
    );
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));
    
    let mut elements: Vec<Rc<dyn Element>> = Vec::new();

    elements.push(Rc::new(
        Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground.clone(),
        )
    ));
    elements.push(Rc::new(
        MovingSphere::new(
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, -1.0),
            0.0,
            1.0,
            0.5,
            material_centre.clone(),
        )
    ));
    elements.push(Rc::new(
        Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left.clone(),
        )
    ));
    elements.push(Rc::new(
        Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right.clone(),
        )
    ));    

    let world = BvhNode::from_elements(elements, 0.0, 1.0);

    Box::new(world)
}

fn create_world1() -> Box<dyn Element> {
    let material_noise = Rc::new(
        Lambertian::new(
            Rc::new(Noise::new(Perlin::new(), 4.0))
        )
    );

    let mut elements: Vec<Rc<dyn Element>> = Vec::new();

    elements.push(Rc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_noise.clone(),
        )
    ));
    elements.push(Rc::new(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            material_noise.clone(),
        )
    ));

    let world = BvhNode::from_elements(elements, 0.0, 1.0);

    Box::new(world)
}

fn ray_colour(ray: &Ray, world: &Box<dyn Element>, depth: isize) -> Colour {
    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);

    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }
    
    if let Some(hit_record) = world.hit(&ray, 0.001, f32::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(&ray, &hit_record) {
            return ray_colour(&scattered, world, depth - 1) * attenuation;
        }
        
        return Colour::new(0.0, 0.0, 0.0);
    }
    
    let unit = ray.direction.normalise();
    let t = 0.5 * (unit.y + 1.0);

    (white * (1.0 - t)) + (blue * t)
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
