use raytracing::math::interval::Interval;
use raytracing::math::ray::Ray;
use raytracing::math::vec3::Vec3;
use raytracing::camera::Camera;

use crate::color::Color;
use crate::hittable::Hittable;
use crate::image::Image;

fn get_pixel_color(ray: Ray, scene: &dyn Hittable) -> Color {
    if let Some(hit) = scene.hit(ray, Interval::new(0.0, f64::INFINITY)) {
        let direction = Vec3::random_on_hemisphere(hit.normal);
        get_pixel_color(Ray::new(hit.point, direction), scene) * 0.5
    } else {
        let direction = ray.direction.unit_vector();
        let intensity = (direction.y + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - intensity) + Color::new(0.5, 0.7, 1.0) * intensity
    }
}

/// Render the scene to an image.
#[must_use]
pub fn render_scene(camera: Camera, scene: &dyn Hittable) -> Image {
    let viewport_u = Vec3::new(camera.viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -camera.viewport_height, 0.0);

    // It is convenient to have these vectors so that we do not need to perform division in the hot
    // loop.
    let viewport_delta_u = viewport_u / raytracing::IMAGE_WIDTH as f64;
    let viewport_delta_v = viewport_v / raytracing::IMAGE_HEIGHT as f64;

    let viewport_upper_left = camera.position
        - Vec3::new(0.0, 0.0, camera.focal_length)
        - (viewport_u + viewport_v) / 2.0;

    let start_pos = viewport_upper_left + (viewport_delta_u + viewport_delta_v) / 2.0;

    let mut image = Image::blank(raytracing::IMAGE_WIDTH, raytracing::IMAGE_HEIGHT);

    for y in 0..raytracing::IMAGE_HEIGHT {
        for x in 0..raytracing::IMAGE_WIDTH {
            // Cast a ray toward the scene... but what ray?
            // Cast it toward a position in the viewport.
            let current_delta_u = viewport_delta_u * x as f64;
            let current_delta_v = viewport_delta_v * y as f64;
            let pixel_center = start_pos + current_delta_u + current_delta_v;
            let ray = Ray::new(camera.position, pixel_center - camera.position);
            let color = get_pixel_color(ray, scene);

            image.set_pixel(x, y, color);
        }
    }

    image
}
