#!/bin/sh

window_width=1600
window_height=900

cargo run --quiet > image.ppm

# These settings make the window big enough that we can see what the renderer
# drew without squinting.
feh \
  --geometry ${window_width}x${window_height} \
  --auto-zoom \
  --force-aliasing \
  image.ppm
