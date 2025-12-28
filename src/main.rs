use std::io::Write;

use crate::color::Color;

mod color;
mod ray;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {:<3}", image_height - j);
        std::io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            color::write_color(std::io::stdout(), pixel_color);
        }
    }

    // These spaces are needed so that none of the characters from "Scanlines remaining" message are left behind.
    eprintln!("\rDone.                   ");
}
