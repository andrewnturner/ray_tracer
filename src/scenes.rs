use std::rc::Rc;

use crate::geometry::point::Point3;
use crate::graphics::colour::Colour;
use crate::render::element::Element;
use crate::render::elements::bvh_node::BvhNode;
use crate::render::elements::element_list::ElementList;
use crate::render::elements::moving_sphere::MovingSphere;
use crate::render::elements::sphere::Sphere;
use crate::render::materials::dielectric::Dielectric;
use crate::render::materials::diffuse_light::DiffuseLight;
use crate::render::materials::lambertian::Lambertian;
use crate::render::materials::metal::Metal;
use crate::render::textures::checker::Checker;
use crate::render::textures::image_texture::ImageTexture;
use crate::render::textures::marble::Marble;
use crate::render::textures::noise::Noise;
use crate::render::textures::solid_colour::SolidColour;
use crate::util::perlin::Perlin;

pub fn create_basic_spheres() -> Box<dyn Element> {
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

pub fn create_noise_spheres() -> Box<dyn Element> {
    let material_noise = Rc::new(
        Lambertian::new(
            Rc::new(Noise::new(Perlin::new(), 4.0))
        )
    );
    let material_marble = Rc::new(
        Lambertian::new(
            Rc::new(Marble::new(Perlin::new(), 4.0))
        )
    );

    let mut elements: Vec<Rc<dyn Element>> = Vec::new();

    elements.push(Rc::new(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_marble.clone(),
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

pub fn create_globe() -> Box<dyn Element> {
    let material_earth = Rc::new(Lambertian::new(
        Rc::new(ImageTexture::new_from_filename("earth.jpg"))
    ));

    let mut world = ElementList::new();

    world.add(
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            2.0,
            material_earth,
        )),
    );

    Box::new(world)
}

pub fn create_lit_globe() -> Box<dyn Element> {
    let material_earth = Rc::new(Lambertian::new(
        Rc::new(ImageTexture::new_from_filename("earth.jpg"))
    ));
    let material_ground = Rc::new(Lambertian::new(
        Rc::new(SolidColour::new(Colour::new(1.0, 0.0, 0.0)))
    ));
    let material_light = Rc::new(DiffuseLight::new(
        Rc::new(SolidColour::new(Colour::new(4.0, 4.0, 4.0)))
    ));

    let mut world = ElementList::new();

    world.add(
        Box::new(Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            material_earth,
        )),
    );
    world.add(
        Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        ))
    );

    world.add(
        Box::new(Sphere::new(
            Point3::new(5.0, 5.0, 5.0),
            1.0,
            material_light,
        ))
    );

    Box::new(world)
}
