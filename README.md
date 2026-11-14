# Kalt

**Evaluate sliced matrix expressions directly in Typst.**

Kalt evaluates complex, vector, and matrix expressions written in Typst math. Its indexing and slicing let you build calculations over matching regions of a matrix, then evaluate the complete expression with `comp(...)`.

## First use: a 2D discrete Laplacian

Given a sampled 5×5 scalar field, compute the five-point discrete Laplacian at all nine interior cells. Each slice selects one aligned 3×3 window: north, south, west, east, and center.

~~~typst
#import "@preview/kalt:0.1.1": comp

#let field = $mat(
  10, 12, 15, 14, 11;
  13, 20, 25, 22, 15;
  14, 24, 35, 28, 16;
  12, 19, 27, 23, 14;
   9, 11, 16, 15, 10
)$
#let h2 = 0.25

// Matching 3×3 windows around every interior cell.
#let north = $[0:3, 1:4]$
#let south = $[2:5, 1:4]$
#let west  = $[1:4, 0:3]$
#let east  = $[1:4, 2:5]$
#let center = $[1:4, 1:4]$

#let laplacian = comp($(
  #field#north + #field#south + #field#west + #field#east
  - 4 * #field#center
) / #h2$)

$ nabla_h^2 T = #laplacian $
~~~

![Typst output of the sliced 2D Laplacian calculation](typst/assets/guide/laplacian-slices.svg)

The result is a 3×3 matrix. Kalt applies the matrix slices and element-wise arithmetic as one expression; the stop indices are exclusive, so each window has matching dimensions.

## Documentation

The [Typst guide and operation reference](typst/README.md) explains:

- matrix and vector notation, arithmetic, dot and cross products
- indexing, ranges, negative indices, and multidimensional slicing
- `comp`, `map`, `reduce`, `any`, and `all`
- complex numbers, mathematical functions, comparisons, and bitwise operators

## Continue

- [Kalt on Typst Universe](https://typst.app/universe/package/kalt/)
- [Report a bug or request a feature](https://github.com/Uhrendoktor/kalt/issues)

Kalt is released under the [MIT License](LICENSE).
