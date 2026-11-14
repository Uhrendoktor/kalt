# Kalt

**Evaluate real, complex, vector, and matrix expressions directly in Typst.**

Kalt evaluates Typst math with scalar and complex arithmetic, vectors, matrices, indexing, and mathematical functions. The compact examples below show the same Typst file as source on the left and as a rendered result on the right.

Each preview is composed by Typst itself. The left pane reads and syntax-highlights the example file; the right pane includes and evaluates that exact file. There are no hand-composed image panels.

## Scalar arithmetic and functions

Binary arithmetic, powers, logarithms, magnitude, rounding, and absolute-value operations.

![Typst source and rendered scalar operations](typst/assets/readme/scalar-operations.svg)

## Matrix operations

Element-wise addition, subtraction, multiplication and division, plus matrix powers, transpose, and matrix multiplication with `dot`.

![Typst source and rendered matrix operations](typst/assets/readme/matrix-operations.svg)

## Vector products

Transpose a column vector for a dot product; use the cross product and `abs` for vector results and lengths.

![Typst source and rendered vector operations](typst/assets/readme/vector-operations.svg)

## Complex arithmetic

Divide complex values, take a conjugate, and calculate complex power and its magnitude.

![Typst source and rendered complex arithmetic](typst/assets/readme/complex-arithmetic.svg)

## Documentation

The [Typst operation reference](typst/README.md) documents the full supported syntax, including comparison and bitwise operators. The short, editable showcase files live in [`typst/docs/examples/`](typst/docs/examples/); one shared Typst layout is defined in [`typst/docs/paired-preview.typ`](typst/docs/paired-preview.typ).

## Build

```bash
cargo build --workspace
cargo build --release --target wasm32-unknown-unknown -p kalt_bindings
```

Regenerate the paired previews with Typst 0.15.1:

```bash
bash typst/docs/render-examples.sh
```

Kalt is released under the [MIT License](LICENSE).
