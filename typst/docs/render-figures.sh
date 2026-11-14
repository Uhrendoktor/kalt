#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/readme}"
mkdir -p "$output_dir"

for example in field-stencil rigid-transform sensor-calibration ac-power; do
  typst compile \
    --root . \
    --format svg \
    "typst/docs/examples/$example.typ" \
    "$output_dir/$example.svg"
done
