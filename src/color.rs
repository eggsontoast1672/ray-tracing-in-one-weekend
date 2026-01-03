use raytracing::math::Vec3;
use raytracing::math::interval::Interval;

/// An RGB color.
pub type Color = Vec3;

/// Convert a normalized color to an RGB tuple.
pub fn as_rgb_tuple(color: Color) -> (u8, u8, u8) {
    const UNIT: Interval = Interval::new(0.0, 1.0);
    let scale_component = |x| (UNIT.clamp(x) * 255.0) as u8;

    (
        scale_component(color.x),
        scale_component(color.y),
        scale_component(color.z),
    )
}
