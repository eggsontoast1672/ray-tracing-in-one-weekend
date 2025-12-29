use raytracing::real_camera::Camera;

use crate::hittable::Hittable;

/// Render the scene to an image.
pub fn render_scene(camera: Camera, scene: &dyn Hittable) -> Image {
    for y in 0..raytracing::IMAGE_HEIGHT {
        for x in 0..raytracing::IMAGE_WIDTH {
            // Cast a ray toward the scene... but what ray?
             
        }
    }
}
