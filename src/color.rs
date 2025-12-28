use raytracing::math::vec3::Vec3;

/// Type alias for colors as vectors.
///
/// Each color has a red, green, and blue component which ranges from 0 to 1.
/// Those compontents are represented by x, y, and z in the vector
/// representation, respectively.
pub type Color = Vec3;

/// Write a color to the output stream.
///
/// This function outputs the given color in PPM format to the specified stream.
/// The writer is not taken explicitly by reference here since the streams we
/// are interested in implement write for themselves as well as their reference
/// types.
///
/// # Panics
///
/// This function will panic if the write call fails.
pub fn write_color<Writer>(mut out: Writer, pixel_color: Color)
where
    Writer: std::io::Write,
{
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    let rbyte = (255.0 * r) as u8;
    let gbyte = (255.0 * g) as u8;
    let bbyte = (255.0 * b) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}
