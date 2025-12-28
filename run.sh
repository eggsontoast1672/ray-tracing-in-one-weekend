#!/bin/sh

window_width=800
window_height=800

cargo run > image.ppm

# These settings make the window big enough that we can see what the renderer
# drew without squinting.
feh \
  --geometry ${window_width}x${window_height} \
  --auto-zoom \
  image.ppm
