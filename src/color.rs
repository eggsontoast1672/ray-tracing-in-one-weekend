use raytracing::math::vec3::Vec3;

/// An RGB color.
pub type Color = Vec3;

/// Convert a normalized color to an RGB tuple.
pub const fn as_rgb_tuple(color: Color) -> (u8, u8, u8) {
    (
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    )
}
