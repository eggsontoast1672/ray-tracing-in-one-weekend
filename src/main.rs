use std::rc::Rc;

use raytracing::math::vec3::Point3;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod sphere;

fn main() {
    let world = {
        let mut world = HittableList::new();
        world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        world
    };

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 854;
    camera.render(&world);
}
