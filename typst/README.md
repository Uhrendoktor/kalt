# Kalt

Kalt evaluates nested Typst math expressions and returns the result as Typst content. It is useful when an equation combines complex numbers, matrices, vectors, indexing, and functions that are awkward to evaluate with Typst's built-in `calc` alone.

## Quick start

```typst
#import "@preview/kalt:0.1.1": comp

$comp(sqrt((3 + 4i) (3 - 4i)))$ // 5
$comp(e^(i pi))$                // -1
$comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8))$
```

`comp(body)` evaluates a Typst math expression. Real scalar results are returned as Typst floats. Complex scalars and matrices remain math content. A `1 × 1` matrix is automatically unwrapped to its single value.

## What's included

- **Arithmetic:** element-wise addition, subtraction, multiplication, division, modulo, exponentiation, and factorial.
- **Comparisons and logic:** `<`, `>`, `≤`, `≥`, `=`, `≠`, numeric indicators `0`/`1`, logical `∧`, `∨`, `¬`, and Boolean-semiring product `⊙`.
- **Complex numbers:** complex arithmetic, conjugation, real/imaginary parts, trigonometric and hyperbolic functions, logarithms, and roots.
- **Matrices and vectors:** element-wise operations, matrix/vector products, cross products, transpose, absolute value/vector norm/determinant, indexing, and slicing.
- **Typst helpers:** `map`, `reduce`, `any`, and `all` for applying callbacks to matrix/vector cells.

## Documentation

The [Kalt reference](REFERENCE.md) is the full operation guide. It documents supported syntax, input and shape rules, return values, precedence, edge cases, and examples for every exposed feature.

For implementation notes, run instructions, and tests, see the [repository README](../README.md) and [Rust backend README](../rust/README.md).

## Development and tests

The evaluator is implemented in Rust and exposed to Typst through WebAssembly. Typst-side integration tests live in `tests`; shared table-driven test helpers are in `tests/utils.typ`.

When changing an operation, update its reference entry and add coverage through the public Typst interface. Errors and incompatible shapes should produce inline, actionable Typst error content.
