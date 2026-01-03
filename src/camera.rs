use crate::math::Vec3;

/// A view into the scene.
pub struct Camera {
    pub position: Vec3,
    pub focal_length: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
}

impl Camera {
    /// Create a new camera at the specified position.
    pub fn new(position: Vec3) -> Self {
        // For now, we have hard coded focal length and viewport dimensions. I am unsure of how I
        // want the API for configuring those aspects of the camera to look.
        let viewport_height = 2.0;

        Self {
            position,
            focal_length: 1.0,
            viewport_width: viewport_height * crate::REAL_ASPECT_RATIO,
            viewport_height,
        }
    }
}
