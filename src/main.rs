use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::math::{Point3, Vec3};

use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

mod color;
mod hittable;
mod hittable_list;
mod image;
mod renderer;
mod sphere;

fn main() -> std::io::Result<()> {
    let camera = Camera {
        position: Vec3::ZERO,
        focal_length: 1.0,
        viewport_height: 2.0,
        viewport_width: 32.0 / 9.0,
    };

    let world = {
        let mut world = HittableList::new();
        world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        world
    };

    let image = renderer::render_scene(camera, &world);
    image.export("image.ppm")
}
