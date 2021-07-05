use std::fs::File;
use std::io::Write;
use std::path::Path;

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
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.25;

            let ir = (255.999 * r) as isize;
            let ig = (255.999 * g) as isize;
            let ib = (255.999 * b) as isize;

            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes());
        }
    }
}
