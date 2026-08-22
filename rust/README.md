# Kalt - Rust Backend

This is the backend Rust library for the Typst package `kalt`. Kalt evaluates
arbitrary nested Typst equations, so there is less need to rely on `calc`.

The backend parses Typst math content into scalar or matrix values and applies
operations through the parser and operation modules. Complex numbers are
represented throughout the evaluation, which allows complex values to occur in
scalars, vectors, matrices, and nested expressions.

This library is standalone and intended for anyone who wants to use the advanced
type handling and parsing capabilities of `kalt` in their own projects.

## Supported Operations

The parser currently exposes the following operations:

### Arithmetic

- `+` — addition; scalar values, scalar/matrix combinations, and equally shaped
  matrices are handled element-wise.
- `-` — subtraction with the same element-wise rules.
- `*` — element-wise multiplication for scalar and matrix values.
- `/` — element-wise division for scalar and matrix values.
- `^` — exponentiation.
- `!` — factorial.
- `binom` — binomial coefficient; matrices must have matching shapes when used
  element-wise.
- `root` / `sqrt` — root operations.
- `abs` — absolute value, complex magnitude, maginute or determinant
- `ceil` — ceiling, applied element-wise to matrices.
- `floor` — floor, applied element-wise to matrices.
- `sign` — sign operation, applied element-wise to matrices.

### Complex Operations

- `conjugate` — complex conjugation.
- `Re` — real part.
- `Im` — imaginary part.

### Matrix and Vector Operations

- `dot` — dot product / matrix multiplication for supported tensor shapes.
- `cross` — three-dimensional vector cross product.
- `transpose` / `T` — matrix transpose.
- indexing — matrix element access, slicing, ranges, reverse ranges, and steps.

### Functions

- `ln` — natural logarithm.
- `log_a` — logarithm with an explicit base.

Operations are implemented over the backend's scalar and matrix tensor types.
Many element-wise operations share the same shape validation and dispatching
machinery, while matrix-specific operations perform their own shape checks. This
is why some combinations that look similar at the Typst level have different
dimensionality requirements.

## Architecture

The main parser lives in `src/parser`. Operations are grouped in
`src/parser/ops`, with individual modules for arithmetic, matrix operations, and
functions. The parser produces tensor values containing either a scalar or a
matrix and applies operations while retaining source spans for error reporting.

The Wasm bindings are separate from this crate. They provide the interface used
by the Typst package while keeping the evaluator itself usable as a standalone
Rust library.

For an implementation example, take a look at the Typst bindings for `kalt`
(available in git).

## Tests

The Typst test suite lives in `../typst/tests` and exercises the backend through
the same interface used by the package. Operation tests are grouped under
`typst/tests/ops`, with shared helpers in `typst/tests/utils.typ`.

When a backend operation is added or changed, update the corresponding Typst
operation test and the test manifest so that the supported-operation coverage
stays visible during review.
