#!/bin/sh

# 1280x720  720p

window_width=1280
window_height=720

# These settings make the window big enough that we can see what the renderer
# drew without squinting.
feh \
  --geometry ${window_width}x${window_height} \
  --auto-zoom \
  --force-aliasing \
  image.ppm &
