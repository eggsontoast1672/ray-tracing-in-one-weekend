use std::io::Write;

use raytracing::math::interval::Interval;
use raytracing::math::ray::Ray;
use raytracing::math::vec3::{Point3, Vec3};

use crate::color::{self, Color};
use crate::hittable::Hittable;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {:<3}", self.image_height - j);
            std::io::stderr().flush().unwrap();

            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * i as f64)
                    + (self.pixel_delta_v * j as f64);

                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(ray, world);
                color::write_color(std::io::stdout(), pixel_color);
            }
        }

        // These spaces are needed so that none of the characters from "Scanlines remaining" message are left behind.
        eprintln!("\rDone.                   ");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = Point3::zero();

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, ray: Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = ray.direction.unit_vector();
        let a = (unit_direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }
}
