// Typst-side API tests registered in the shared CI test report.

#import "../lib.typ": all, any, comp, map
#import "utils.typ": assert-typst-eq, assert-eq

#assert-typst-eq("comp real scalar returns float", type(comp($2 + 3$)), "float")

#assert-eq(
  "map callback receives x and y coordinates",
  map((value, x, y) => $ #value + #x + #y $, $mat(1, 2; 3, 4)$),
  $mat(1, 3; 4, 6)$,
)

#let highlighted = map(
  (value, x, y) => highlight(value, fill: yellow),
  $mat(1, 2)$,
)
#assert-typst-eq("map preserves highlighted content", "highlight" in repr(highlighted), true)

#assert-typst-eq("any finds matching cell", any(value => comp(value) > 3, $mat(1, 2; 3, 4)$), true)
#assert-typst-eq("any rejects no matches", any(value => comp(value) > 9, $mat(1, 2; 3, 4)$), false)
#assert-typst-eq("all accepts every matching cell", all(value => comp(value) > 0, $mat(1, 2; 3, 4)$), true)
#assert-typst-eq("all rejects a failing cell", all(value => comp(value) > 1, $mat(1, 2; 3, 4)$), false)
