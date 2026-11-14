# Changelog

## [0.1.2] - 2026-11-14

### Added
- Added element-wise matrix and vector operations, including cell-wise arithmetic and shape validation.
- Added matrix helpers: `map`, `reduce`, `any`, `all`, and `tensor-variant`.
- Added modulo, bitwise AND/OR/XOR/NOT, boolean matrix products, and comparison and logical operators.
- Added trigonometric and inverse-trigonometric functions, including `sin`, `cos`, `tan`, and their hyperbolic variants.
- Added expanded matrix and vector indexing, slicing, and selection support.
- Added new Typst examples and API/operation tests.

### Improved
- Improved expression parsing and operator precedence, including non-associative comparisons and composed operator aliases.
- Improved calculation results: real scalar results are returned as Typst floats, and 1×1 matrix results are unwrapped to scalars.
- Improved error reporting with more precise source spans and contextual hints.
- Expanded and reorganized the package documentation with examples for scalar, matrix, vector, and complex-number calculations.

### Maintenance
- Updated package and workspace versions to 0.1.2 and refreshed Rust dependency metadata.
- Refreshed documentation illustrations and examples.

## [0.1.1]

- Previous release.
