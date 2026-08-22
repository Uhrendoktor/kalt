# Kalt

Kalt evaluates arbitrary nested Typst equations and returns the result as Typst
content. It is meant for cases where `calc` gets awkward: deeply nested
expressions, complex numbers, matrices, vectors, and indexing.

Since the result is still Typst content, you can keep formatting, reusing, and
combining it in your document instead of ending up with a dead numeric result.

The repository is split into a Rust backend, Wasm bindings, and the Typst-side
package. If you want the user-facing examples, start with
[typst/README.md](typst/README.md).

## What It Covers

Kalt currently supports the following operations. Operations work on scalars
and, where applicable, on matrices element-wise.

### Arithmetic

- `+` / `add` — addition of scalars, scalar-matrix pairs, and equally shaped
  matrices.
- `-` / `sub` — subtraction with the same scalar and element-wise matrix rules.
- `*` / `mul` — element-wise multiplication. Scalar-matrix multiplication is
  also applied element-wise.
- `/` / `div` — element-wise division. Scalar-matrix division follows the
  element-wise rules of the backend.
- `^` / `pow` — exponentiation. Scalar powers and supported matrix powers are
  evaluated by the backend.
- `!` / `factorial` — factorial for supported numeric values.
- `binom` — binomial coefficients, including element-wise operation on equally
  shaped matrices.
- `root` / `sqrt` — roots and square roots.
- `abs` — absolute value / complex magnitude, magnitude or determinant.
- `ceil` — ceiling, applied element-wise to matrices.
- `floor` — floor, applied element-wise to matrices.
- `sign` — sign operation, applied element-wise to matrices.

### Complex Numbers

- `conjugate` — complex conjugation.
- `Re` — real part.
- `Im` — imaginary part.

Complex numbers can occur anywhere in an expression and can be combined with
scalars and matrices.

### Matrix and Vector Operations

- `dot` — matrix/vector dot product and matrix multiplication according to the
  supported tensor shapes.
- `cross` — cross product for three-dimensional vectors.
- `transpose` / `T` — matrix transpose.
- `index` — matrix indexing, including slices and reverse/step ranges.

### Functions

- `ln` — natural logarithm.
- `log_a` — logarithm with an explicit base `a`.

The operation parser combines these operations into nested expressions, so an
operation can generally be used as part of another operation without first
materialising an intermediate Typst value.

## Layout

- [rust](rust) contains the `kalt` crate, which does the actual evaluation.
- [rust_bindings](rust_bindings) builds the Wasm wrapper used by Typst.
- [typst](typst) has the package README, examples, tests, and assets.

## Build

From the repo root:

```bash
cargo build --workspace
```

To build the bindings only:

```bash
cargo build --release --target wasm32-unknown-unknown -p kalt_bindings
```

## Docs

- [Rust README](rust/README.md)
- [Typst README](typst/README.md)
