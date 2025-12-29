/// An RGB color.
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, b, g }
    }

    /// Create a new color from normalized components.
    pub const fn rgb_normalized(r: f64, g: f64, b: f64) -> Self {
        let r = (r * 255.0) as u8;
        let g = (g * 255.0) as u8;
        let b = (b * 255.0) as u8;

        Self::rgb(r, g, b)
    }
}
