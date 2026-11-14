// Tests the public Typst API, including scalar conversions, direct math results, and matrix helpers.

#import "../lib.typ": all, any, comp, map, reduce
#import "utils.typ": assert-typst-eq, assert-eq

#assert-typst-eq("comp real scalar returns float", type(comp($2 + 3$)), float)
#assert-typst-eq("comp 1x1 real matrix returns float", type(comp($mat(7)$)), float)
#assert-typst-eq("comp 1x1 real matrix unwraps to scalar", comp($mat(7)$), 7.0)
#assert-typst-eq("comp 1x1 complex matrix returns content", type(comp($mat(7i)$)), content)

#assert-typst-eq("comp direct integer returns float", comp(7), 7.0)
#assert-typst-eq("comp direct float returns float", comp(2.5), 2.5)
#assert-typst-eq("comp direct string parses radix integer", comp("0xFF"), 255.0)
#assert-typst-eq("comp direct decimal returns float", comp(decimal("2.5")), 2.5)
#assert-typst-eq("comp direct 1x1 matrix returns float", comp(math.mat(..((7,),))), 7.0)
#let matrix-result = comp(math.mat(..((1, 2), (3, 4))))
#assert-typst-eq("comp returns matrix content directly", matrix-result.func(), math.mat)

#let vector-result = comp(math.vec($1$, $2$, $3$))
#assert-typst-eq("comp returns vector content directly", vector-result.func(), math.vec)
#assert-typst-eq(
  "comp evaluates a transposed column-vector dot product",
  comp($vec(1, 2, 3)^T dot vec(4, 5, 6)$),
  32.0,
)

#assert-eq(
  "map passes numeric cells to callback as floats",
  map(
    (value, x, y) => if type(value) == float { $1$ } else { $2$ },
    $mat(1, 2)$,
  ),
  $mat(1, 1)$,
)

#assert-eq(
  "map callback receives x and y coordinates",
  map((value, x, y) => $ #value + #x + #y $, $mat(1, 2; 3, 4)$),
  $mat(1, 3; 4, 6)$,
)

#let highlighted = map(
  (value, x, y) => highlight(str(value), fill: yellow),
  $mat(1, 2)$,
)
#assert-typst-eq("map preserves highlighted content", "highlight" in repr(highlighted), true)

#assert-typst-eq("any finds matching cell", any(value => comp(value) > 3, $mat(1, 2; 3, 4)$), true)
#assert-typst-eq("any rejects no matches", any(value => comp(value) > 9, $mat(1, 2; 3, 4)$), false)
#assert-typst-eq("all accepts every matching cell", all(value => comp(value) > 0, $mat(1, 2; 3, 4)$), true)
#assert-typst-eq("all rejects a failing cell", all(value => comp(value) > 1, $mat(1, 2; 3, 4)$), false)

#let stencil-field = $mat(
  10, 12, 15, 14, 11;
  13, 20, 25, 22, 15;
  14, 24, 35, 28, 16;
  12, 19, 27, 23, 14;
  9, 11, 16, 15, 10
)$
#let stencil-north = comp($#stencil-field[0:3, 1:4]$)
#let stencil-south = comp($#stencil-field[2:5, 1:4]$)
#let stencil-west = comp($#stencil-field[1:4, 0:3]$)
#let stencil-east = comp($#stencil-field[1:4, 2:5]$)
#let stencil-center = comp($#stencil-field[1:4, 1:4]$)
#let stencil-laplacian = map(
  (n, s, w, e, c, _x, _y) => (n + s + w + e - 4 * c) / 0.25,
  stencil-north, stencil-south, stencil-west, stencil-east, stencil-center,
)
#assert-eq(
  "map combines indexed matrix slices into a 2D finite-difference stencil",
  stencil-laplacian,
  $mat(-24, -32, -24; -32, -144, -64; -8, -60, -32)$,
)
