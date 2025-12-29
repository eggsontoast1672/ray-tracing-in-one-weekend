pub mod math;

/// Return a random real in `[0, 1)`.
pub fn random_f64() -> f64 {
    rand::random_range(0.0..1.0)
}

/// Return a random real in `[min, max)`.
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
