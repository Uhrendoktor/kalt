// Public Typst API behavior tests.

#import "../lib.typ": all, any, comp, map
#import "utils.typ": assert-typst-eq, assert-eq

#assert-typst-eq("comp real scalar returns float", type(comp($2 + 3$)), float)
#assert-typst-eq("comp 1x1 real matrix returns float", type(comp($mat(7)$)), float)
#assert-eq("comp 1x1 real matrix unwraps to scalar", comp($mat(7)$), 7)
#assert-typst-eq("comp 1x1 complex matrix returns content", type(comp($mat(7i)$)), content)

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
