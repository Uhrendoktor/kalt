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

// Keep the README's temperature-conversion example executable in the API suite.
#let fahrenheit = $mat(68, 77; 86, 95)$
#let celsius = map(value => (value - 32) * 5 / 9, fahrenheit)
#assert-eq("map converts Fahrenheit readings to Celsius", celsius, $mat(20, 25; 30, 35)$)

#let average = reduce((sum, value) => sum + comp(value), 0, celsius) / 4
#assert-typst-eq("reduce calculates the average temperature", average, 27.5)
#assert-typst-eq("any finds a reading at least 30 Celsius", any(value => comp(value) >= 30, celsius), true)
#assert-typst-eq("all readings are above freezing", all(value => comp(value) > 0, celsius), true)

#table(
  columns: 2,
  [*Measurement*], [*Values*],
  [Fahrenheit], fahrenheit,
  [Celsius], celsius,
  [Average], [#average °C],
  [Any reading at least 30 °C?], [#any(value => comp(value) >= 30, celsius)],
  [All readings above 0 °C?], [#all(value => comp(value) > 0, celsius)],
)
