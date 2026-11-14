#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/guide}"
mkdir -p "$output_dir"

figures=(
  matrix-input
  matrix-expansion
  matrix-result
  vector-dot
  vector-cross
  complex-product
  temperature-map
  temperature-average
  temperature-predicates
)

for figure in "${figures[@]}"; do
  typst compile \
    --root . \
    --format svg \
    --input "figure=$figure" \
    typst/docs/figures.typ \
    "$output_dir/$figure.svg"
done
