// Typst-side helper tests for map, any and all.

#import "../lib.typ": all, any, comp, map
#import "utils.typ": assert-eq

#assert(type(comp($2 + 3$)) == float)

#assert-eq(
  "map callback receives zero-based row-major index",
  map((value, index) => $#value + #index$, $mat(1, 2; 3, 4)$),
  $mat(1, 3; 5, 7)$,
)

#let highlighted = map(
  (value, index) => highlight(value, fill: yellow),
  $mat(1, 2)$,
)
#assert("highlight" in repr(highlighted))

#assert(any(value => comp(value) > 3, $mat(1, 2; 3, 4)$))
#assert(not any(value => comp(value) > 9, $mat(1, 2; 3, 4)$))
#assert(all(value => comp(value) > 0, $mat(1, 2; 3, 4)$))
#assert(not all(value => comp(value) > 1, $mat(1, 2; 3, 4)$))
