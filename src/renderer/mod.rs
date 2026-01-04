use raytracing::camera::Camera;
use raytracing::color::Color;
use raytracing::image::Image;
use raytracing::math::Vec3;
use raytracing::math::interval::Interval;
use raytracing::math::ray::Ray;
use raytracing::ui;

use crate::hittable::Hittable;

pub mod material;

/// Get a random vector in the `[-0.5, 0.5]^2` product space.
fn sample_square() -> Vec3 {
    Vec3::new(
        raytracing::random_f64() - 0.5,
        raytracing::random_f64() - 0.5,
        0.0,
    )
}

fn get_ray(
    x: u16,
    y: u16,
    viewport_delta_u: Vec3,
    viewport_delta_v: Vec3,
    start_pos: Vec3,
    eye: Vec3,
) -> Ray {
    let offset = sample_square();

    let current_delta_u = viewport_delta_u * (x as f64 + offset.x);
    let current_delta_v = viewport_delta_v * (y as f64 + offset.y);
    let pixel_center = start_pos + current_delta_u + current_delta_v;
    Ray::new(eye, pixel_center - eye)
}

fn get_pixel_color(ray: Ray, depth: i32, scene: &dyn Hittable) -> Color {
    const EPSILON: f64 = 0.001;

    if depth <= 0 {
        return Color::ZERO;
    }

    if let Some(hit) = scene.hit(ray, Interval::new(EPSILON, f64::INFINITY)) {
        return if let Some(scattered) = hit.clone().material.scatter(ray, hit) {
            let next_color = get_pixel_color(scattered.ray, depth - 1, scene);

            Color::new(
                scattered.attenuation.x * next_color.x,
                scattered.attenuation.y * next_color.y,
                scattered.attenuation.z * next_color.z,
            )
        } else {
            Color::ZERO
        };
    }

    let direction = ray.direction.unit_vector();
    let intensity = (direction.y + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - intensity) + Color::new(0.5, 0.7, 1.0) * intensity
}

/// Render the scene to an image.
#[must_use]
pub fn render_scene<I>(camera: Camera, scene: &dyn Hittable) -> I
where
    I: Image,
{
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

    let pixel_samples_scale = 1.0 / raytracing::SAMPLES_PER_PIXEL as f64;

    let mut image = I::blank(raytracing::IMAGE_WIDTH, raytracing::IMAGE_HEIGHT);

    let one_percent = 1.0 / raytracing::IMAGE_HEIGHT as f64;

    for y in 0..raytracing::IMAGE_HEIGHT {
        ui::update(y as f64 * one_percent);

        for x in 0..raytracing::IMAGE_WIDTH {
            let mut color = Color::ZERO;
            for _ in 0..raytracing::SAMPLES_PER_PIXEL {
                let ray = get_ray(
                    x,
                    y,
                    viewport_delta_u,
                    viewport_delta_v,
                    start_pos,
                    camera.position,
                );
                color += get_pixel_color(ray, raytracing::MAX_DEPTH, scene);
            }

            image.set_pixel(x, y, color * pixel_samples_scale);
        }
    }

    ui::finish();

    image
}
