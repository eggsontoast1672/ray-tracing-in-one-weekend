use std::io::Write;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

mod color;
mod ray;
mod vec3;

/// Check if a ray intersects a sphere.
///
/// This function returns true if the given ray intersects the sphere with given center and radius
/// for absolutely any value of t, and false otherwise. Since the return value is only a boolean,
/// this function is not that useful for rendering things that look nice.
fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.length_squared();
    let b = ray.direction.dot(oc) * -2.0;
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    discriminant >= 0.0
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

// This section computes the width and height of the image itself. PPM images must have integer
// dimensions, which is the reason we need to compute them in this way.
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = {
    let height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    if height < 1 { 1 } else { height }
};

// This section has to do with the camera. The focal length is the distance from the eye (camera)
// to the center of the viewport. This time, we fix the viewport's height and use the aspect ratio
// to compute the width. Note that we cannot just use the aspect ratio above, as that is only the
// ideal aspect ratio. The actual aspect ratio is computed using the true integer dimensions of the
// window.
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
const CAMERA_CENTER: Point3 = Point3::zero();

const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

fn main() {
    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {:<3}", IMAGE_HEIGHT - j);
        std::io::stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_direction);

            let pixel_color = ray_color(r);
            color::write_color(std::io::stdout(), pixel_color);
        }
    }

    // These spaces are needed so that none of the characters from "Scanlines remaining" message are left behind.
    eprintln!("\rDone.                   ");
}
