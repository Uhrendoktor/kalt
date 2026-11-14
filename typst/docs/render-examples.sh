#!/usr/bin/env bash
set -euo pipefail

output_dir="${1:-typst/assets/readme}"
mkdir -p "$output_dir"

package_version="$(sed -n 's/^version = "\(.*\)"/\1/p' typst/typst.toml)"
if [[ -z "$package_version" ]]; then
  echo "Could not read package version from typst/typst.toml" >&2
  exit 1
fi

# Render through the same public import users copy from the README.
package_path="$(mktemp -d)"
package_root="$package_path/preview/kalt/$package_version"
mkdir -p "$package_root"
cp -a typst/. "$package_root/"
export TYPST_PACKAGE_PATH="$package_path"

for example in scalar-operations matrix-operations vector-operations complex-arithmetic; do
  echo "Rendering $example"
  typst compile \
    --package-path "$package_path" \
    --root . \
    --format svg \
    "typst/docs/examples/$example.typ" \
    "$output_dir/$example.svg"
done

# Compile the non-visual collection-helper example as a documentation check.
echo "Checking collection-helpers"
typst compile \
  --package-path "$package_path" \
  --root . \
  "typst/docs/examples/collection-helpers.typ" \
  "$package_path/collection-helpers.pdf"

# This example is intentionally invalid: verify that Typst shows the helpful shape error.
echo "Checking error hints"
error_log="$package_path/error-hints.log"
if typst compile \
  --package-path "$package_path" \
  --root . \
  "typst/docs/examples/error-hints.typ" \
  "$package_path/error-hints.pdf" >"$error_log" 2>&1; then
  echo "The error-hints example unexpectedly compiled successfully." >&2
  exit 1
fi
if ! grep -Fq "dot product can only be applied to matrices with compatible shapes" "$error_log"; then
  cat "$error_log" >&2
  echo "The expected matrix-shape error message was not found." >&2
  exit 1
fi
