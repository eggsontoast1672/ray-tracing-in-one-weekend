use std::io::Write;
use std::path::Path;

use crate::color::{self, Color};
use crate::image::Image;

pub struct PixmapImage {
    width: u16,
    height: u16,
    pixel_data: Box<[u8]>,
}

impl Image for PixmapImage {
    fn blank(width: u16, height: u16) -> Self {
        let data_size = width as usize * height as usize * 3;
        let pixel_data = vec![0; data_size].into_boxed_slice();

        Self {
            width,
            height,
            pixel_data,
        }
    }

    fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        if x >= self.width {
            panic!("the x is {} but the width is {}", x, self.width);
        } else if y >= self.height {
            panic!("the y is {} but the height is {}", y, self.height);
        }

        let index = (y as usize * self.width as usize + x as usize) * 3;
        let (r, g, b) = color::as_rgb_tuple(color);

        self.pixel_data[index] = r;
        self.pixel_data[index + 1] = g;
        self.pixel_data[index + 2] = b;
    }

    fn export<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;

        writeln!(file, "P3\n{} {}\n255", self.width, self.height)?;

        for pixel_index in 0..self.width as usize * self.height as usize {
            let byte_index = pixel_index * 3;
            let r = self.pixel_data[byte_index];
            let g = self.pixel_data[byte_index + 1];
            let b = self.pixel_data[byte_index + 2];

            writeln!(file, "{} {} {}", r, g, b)?;
        }

        Ok(())
    }
}
