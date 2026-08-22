# Kalt - Rust Backend

This is the backend Rust library for the Typst package `kalt`. Kalt evaluates
arbitrary nested Typst equations, so there is less need to rely on `calc`.

The backend parses Typst math content into scalar or matrix values and applies
operations through the parser and operation modules. Complex numbers are
represented throughout the evaluation, which allows complex values to occur in
scalars, vectors, matrices, and nested expressions.

This library is standalone and intended for anyone who wants to use the advanced
type handling and parsing capabilities of `kalt` in their own projects.

For the user-facing list of supported operations and examples, see the
[Typst README](../typst/README.md).

## Architecture

The main parser lives in `src/parser`. Operations are grouped in
`src/parser/ops`, with individual modules for arithmetic, matrix operations, and
functions. The parser produces tensor values containing either a scalar or a
matrix and applies operations while retaining source spans for error reporting.

Operations that work element-wise share common dispatch and shape-validation
machinery. Matrix-specific operations perform their own dimensionality checks.
This allows the backend to report useful errors for incompatible tensor shapes
instead of relying on generic arithmetic failures.

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
