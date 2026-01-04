# Ray Tracing in One Weekend

This repository holds my code for the [*Ray Tracing in One Weekend*][1] book. I
decided to follow along in Rust because, why not? In this document, I plan to
write down some information on concepts that I am learning so that they stick a
bit better. A side effect of that is that the project is better documented!

[1]: https://raytracing.github.io/books/RayTracingInOneWeekend.html

## The PPM Format

The PPM image format is a simple text or binary based format for storing image
data, with emphasis on **simple**. Not only is it meant to be easy to
understand, but it is also very easy to parse. This is the image format that
the renderer will output.

Every PPM file starts with a two byte magic number which indicates whether the
file is in an ascii or binary format. If the first two bytes are `P3`, the file
is plain ASCII PPM data. On the other hand, if the first two bytes are `P6` (or
`0x5036` in raw bytes) then the format is taken to be raw binary data. We will
only be using the ASCII format for this project, so there will be no further
mention of `P6`.

The next two numbers in the file are the dimensions of the image, in pixels.
The first number represents the width and the second represents the height, and
they should be separated by a space. The next number after the dimensions is
the number of bits per component. For example, if you only wanted pure red,
green or blue, you could set this number to 1.

The remainder of the file is the actual image data. The color of each pixel is
determined by three numbers, each one responsible for the red, green, or blue
component respectively. A complete example taken from the [Netpbm Wikipedia][2]
page follows:

[2]: https://en.wikipedia.org/wiki/Netpbm

```ppm
P3
# "P3" means this is a RGB color image in ASCII
# "3 2" is the width and height of the image in pixels
# "255" is the maximum value for each color
# This, up through the "255" line below are the header.
# Everything after that is the image data: RGB triplets.
# In order: red, green, blue, yellow, white, and black.
3 2
255
255   0   0
  0 255   0
  0   0 255
255 255   0
255 255 255
  0   0   0
```

## Bitmap Format

- Bitmap file header (general information)
- DIB header (defines pixel format)
- Image data

- Magic number 0x42 0x4D
- Size of entire file in bytes (little endian, including header)
- Four reserved bytes (typically 0)
- Offset of image data in bytes
- Each row of the image must be 4-byte aligned
- Bytes of image data are little endian (BGR instead of RGB)
