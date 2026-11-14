#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/readme}"
mkdir -p "$output_dir"

for example in field-stencil rigid-transform force-moment ac-power; do
  typst compile \
    --root . \
    --input "example=$example" \
    --format svg \
    typst/docs/paired-preview.typ \
    "$output_dir/$example.svg"
done
