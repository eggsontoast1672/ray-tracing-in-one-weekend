use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct BitmapImage {
    width: u16,
    height: u16,
    pixel_data: Box<[u8]>,
}

impl BitmapImage {
    /// Align the image's width to a four byte boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::bitmap::BitmapImage;
    ///
    /// assert_eq!(BitmapImage::bytes_per_row(0), 0);
    /// assert_eq!(BitmapImage::bytes_per_row(1), 4);
    /// assert_eq!(BitmapImage::bytes_per_row(2), 4);
    /// assert_eq!(BitmapImage::bytes_per_row(3), 4);
    /// ```
    fn bytes_per_row(width: u16) -> usize {
        let unpadded = width as isize * 3;
        let padded = unpadded + (-unpadded).rem_euclid(4);
        padded as usize
    }

    /// Create a new bitmap image.
    pub fn new(width: u16, height: u16) -> Self {
        let data_size = Self::bytes_per_row(width) * height as usize;

        Self {
            width,
            height,
            pixel_data: vec![0; data_size].into_boxed_slice(),
        }
    }

    /// Return the size of the file required to contain this image.
    fn size_bytes(&self) -> u32 {
        const BITMAP_HEADER_SIZE: u32 = 14;
        const DIB_HEADER_SIZE: u32 = 12;

        BITMAP_HEADER_SIZE + DIB_HEADER_SIZE + self.pixel_data.len() as u32
    }

    /// Write the header data to the specified file.
    fn write_header(&self, file: &mut File) -> std::io::Result<()> {
        let file_size = self.size_bytes();

        file.write_all(b"BM")?; // Magic number
        file.write_all(&file_size.to_le_bytes())?;
        file.write_all(&[0x00, 0x00, 0x00, 0x00])?; // Reserved
        file.write_all(&[26, 0x00, 0x00, 0x00])?; // Pixel data offset

        Ok(())
    }

    /// Write the DIB header data to the specified file. The specific DIB header used is the
    /// BITMAPCOREHEADER, and this is hard coded into the method.
    fn write_dib_header(&self, file: &mut File) -> std::io::Result<()> {
        file.write_all(&12_u32.to_le_bytes())?; // DIB header size
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;
        file.write_all(&1_u16.to_le_bytes())?; // Number of color planes (must be 1)
        file.write_all(&24_u16.to_le_bytes())?; // Bits per pixel

        Ok(())
    }

    /// Export the image to a file.
    pub fn export<P>(&self, path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let mut file = File::create(path)?;

        self.write_header(&mut file)?;
        self.write_dib_header(&mut file)?;

        file.write_all(&self.pixel_data)?;

        Ok(())
    }
}
