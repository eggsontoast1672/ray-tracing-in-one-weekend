use std::rc::Rc;

use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::image::Image;
use raytracing::image::bitmap::BitmapImage;
use raytracing::math::{Point3, Vec3};

use crate::hittable_list::HittableList;
use crate::renderer::material::{Lambertian, Metal};
use crate::sphere::Sphere;

mod hittable;
mod hittable_list;
mod renderer;
mod sphere;

fn main() -> std::io::Result<()> {
    let camera = Camera {
        position: Vec3::ZERO,
        focal_length: 1.0,
        viewport_height: 2.0,
        viewport_width: 32.0 / 9.0,
    };

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    });

    let world = {
        let mut world = HittableList::new();
        world.add(Rc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));
        world.add(Rc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )));
        world.add(Rc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )));
        world.add(Rc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )));
        world
    };

    let image: BitmapImage = renderer::render_scene(camera, &world);
    image.export("image.ppm")
}
