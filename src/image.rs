use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::color::{self, Color};

/// A PPM image.
///
/// Once the image is created, the dimensions cannot be changed.
pub struct Image {
    width: usize,
    height: usize,
    bytes: Box<[u8]>,
}

// Invariant: bytes.len() == width * height * 3

impl Image {
    /// Create a blank image with the specified dimensions.
    pub fn blank(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            bytes: vec![0; width * height * 3].into_boxed_slice(),
        }
    }

    /// Return the dimensions of the image.
    pub fn _dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Set the color of a pixel.
    ///
    /// # Panics
    ///
    /// This method will panic if `x >= self.width` or `y >= self.height`.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width {
            panic!("the x is {} but the width is {}", x, self.width);
        }

        if y >= self.height {
            panic!("the y is {} but the height is {}", y, self.height);
        }

        let index = (y * self.width + x) * 3;
        let (r, g, b) = color::as_rgb_tuple(color);

        self.bytes[index] = r;
        self.bytes[index + 1] = g;
        self.bytes[index + 2] = b;
    }

    /// Write the image to a file.
    pub fn export<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(path)?;
        writeln!(file, "P6\n{} {}\n255", self.width, self.height)?;
        file.write_all(&self.bytes)?;
        Ok(())
    }
}
