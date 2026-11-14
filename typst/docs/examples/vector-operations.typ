#import "../../lib.typ": comp

#let a = $vec(1, 2, 3)$
#let b = $vec(4, 5, 6)$
#let dot-product = comp($#a^T dot #b$)
#let cross-product = comp($#a times #b$)
#let length = calc.round(comp($abs(#a)$), digits: 3)
#let cross-length = calc.round(comp($abs(#cross-product)$), digits: 3)

#grid(
  columns: (1fr, 1fr),
  gutter: 4mm,
  $a = #a$,
  $b = #b$,
  $a^T dot b = #dot-product$,
  $a times b = #cross-product$,
  $abs(a) = #length$,
  $abs(a times b) = #cross-length$,
)
