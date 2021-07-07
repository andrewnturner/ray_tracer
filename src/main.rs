mod graphics;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use num_traits::float::Float;
use num_traits::cast::FromPrimitive;

use graphics::colour::Colour;

fn main() {
    let image_width = 256;
    let image_height = 256;

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
            let c = Colour::new(
                (i as f32) / ((image_width - 1) as f32),
                (j as f32) / ((image_height - 1) as f32),
                0.25,
            );

            write_colour(&mut file, &c);
        }
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
