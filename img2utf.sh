#!/bin/sh
#
# Simple wrapper script to use imagemagick for scaling and image suppport.
#

convert -resize '120' $1 out.png
./target/debug/image2linetext out.png
rm out.png
