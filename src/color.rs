use crate::math::Vec3;
use crate::math::interval::Interval;

/// An RGB color.
pub type Color = Vec3;

/// Perform gamma correction on a color component.
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Convert a normalized color to an RGB tuple.
pub fn as_rgb_tuple(color: Color) -> (u8, u8, u8) {
    const UNIT: Interval = Interval::new(0.0, 1.0);
    let scale_component = |x| {
        let gamma_corrected = linear_to_gamma(x);
        (UNIT.clamp(gamma_corrected) * 255.0) as u8
    };

    (
        scale_component(color.x),
        scale_component(color.y),
        scale_component(color.z),
    )
}
