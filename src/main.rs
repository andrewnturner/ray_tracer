mod geometry;
mod graphics;
mod render;
mod util;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use geometry::point::Point3;
use geometry::ray::Ray;
use geometry::vector::Vector3;
use render::element::Element;
use render::elements::element_list::ElementList;
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

    let world = create_world();

    let path = Path::new("render.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create {}: {}", display, why),
    };

    file.write_all("P3\n".as_bytes());
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes());
    file.write_all("255\n".as_bytes());

    for j in (0..image_height).rev() {
        println!("Line {} of {}", j + 1, image_height);

        for i in 0..image_width {
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);

            let r = Ray::new(
                origin,
                (lower_left_corner + (horizontal * u) + (vertical * v)).as_vector3(),
            );

            let pixel_colour = ray_colour(&r, &world);
            write_colour(&mut file, &pixel_colour);
        }
    }
}

fn create_world() -> Box<dyn Element> {
    let mut world = ElementList::new();
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)
    ));
    world.add(Box::new(
        Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)
    ));

    Box::new(world)
}

fn ray_colour(ray: &Ray, world: &Box<dyn Element>) -> Colour {
    let white = Colour::new(1.0, 1.0, 1.0);
    let blue = Colour::new(0.5, 0.7, 1.0);
    
    if let Some(hit_record) = world.hit(&ray, 0.0, f32::INFINITY) {
        let normal = hit_record.normal.normalise();
        return Colour::new(
            normal.x + 1.0,
            normal.y + 1.0,
            normal.z + 1.0,
        ) * 0.5;
    }
    
    let unit = ray.direction.normalise();
    let t = 0.5 * (unit.y + 1.0);

    (white * (1.0 - t)) + (blue * t)
}

fn write_colour(file: &mut File, colour: &Colour) {
    let scale = 255.999;

    let sr = scale * colour.r;
    let sg = scale * colour.g;
    let sb = scale * colour.b;

    let ir = sr as isize;
    let ig = sg as isize;
    let ib = sb as isize;

    file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes());
}
