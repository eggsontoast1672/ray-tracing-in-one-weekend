use std::path::Path;

use crate::color::Color;

pub mod bitmap;
pub mod pixmap;

/// An interface representing some renderable image.
pub trait Image {
    /// Create a blank image of the specified size.
    fn blank(width: u16, height: u16) -> Self;

    /// Set the pixel at the specified location to the desired color.
    fn set_pixel(&mut self, x: u16, y: u16, color: Color);

    /// Export the image to the specified path.
    fn export<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()>;
}
