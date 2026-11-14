#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/guide}"
mkdir -p "$output_dir"

typst compile \
  --root . \
  --format svg \
  typst/docs/figures.typ \
  "$output_dir/laplacian-slices.svg"
