# Kalt

**Evaluate sliced matrix expressions directly in Typst.**

Kalt evaluates complex, vector, and matrix expressions written in Typst math. Its indexing and slicing let you compute over selected regions of a matrix, then combine aligned results with Kalt's collection helpers.

## First use: a 2D discrete Laplacian

Compute the five-point discrete Laplacian of a sampled 5×5 field at all nine interior cells. Each slice selects one 3×3 window: north, south, west, east, or center. `map` combines the corresponding cells and evaluates the stencil.

~~~typst
#import "@preview/kalt:0.1.1": comp, map

#let field = $${mat}$
#let north = comp($${mat}[0:3, 1:4]$)
#let south = comp($${mat}[2:5, 1:4]$)
#let west = comp($${mat}[1:4, 0:3]$)
#let east = comp($${mat}[1:4, 2:5]$)
#let center = comp($${mat}[1:4, 1:4]$)

#let laplacian = map(
  (n, s, w, e, c, _x, _y) => (n + s + w + e - 4 * c) / 0.25,
  north, south, west, east, center,
)

$ nabla_h^2 T = #laplacian $
~~~

![Typst output of the sliced 2D Laplacian calculation](typst/assets/guide/laplacian-slices.svg)

The result is a 3×3 matrix of second-order spatial differences. Slice stop indices are exclusive, so all five windows have matching dimensions.

## Documentation

The [Typst guide and operation reference](typst/README.md) explains matrix and vector notation, arithmetic, indexing and slicing, the collection helpers, complex functions, comparisons, and bitwise operators.

## Continue

- [Kalt on Typst Universe](https://typst.app/universe/package/kalt/)
- [Report a bug or request a feature](https://github.com/Uhrendoktor/kalt/issues)

Kalt is released under the [MIT License](LICENSE).
