#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/readme}"
mkdir -p "$output_dir"

for example in scalar-operations matrix-operations vector-operations complex-arithmetic; do
  typst compile \
    --root . \
    --input "example=$example" \
    --format svg \
    typst/docs/paired-preview.typ \
    "$output_dir/$example.svg"
done
