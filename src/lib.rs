pub mod camera;
pub mod math;
pub mod ui;

pub const TARGET_ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: usize = 400;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / TARGET_ASPECT_RATIO) as usize;
pub const REAL_ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;

/// Return a random real in `[0, 1)`.
pub fn random_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

/// Return a random real in `[min, max)`.
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
