pub mod camera;
pub mod color;
pub mod image;
pub mod math;
pub mod ui;

pub const TARGET_ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u16 = 400;
pub const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / TARGET_ASPECT_RATIO) as u16;
pub const REAL_ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
pub const SAMPLES_PER_PIXEL: i32 = 10;
pub const MAX_DEPTH: i32 = 100;

/// Return a random real in `[0, 1)`.
pub fn random_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

/// Return a random real in `[min, max)`.
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
