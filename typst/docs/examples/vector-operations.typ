#import "../../lib.typ": comp

#let a = $vec(1, 2, 3)$
#let b = $vec(4, 5, 6)$
#let dot-product = comp($#a^T dot #b$)
#let cross-product = comp($#a times #b$)
#let length = comp($abs(#a)$)
#let cross-length = comp($abs(#cross-product)$)

#align(center)[
  $a = #a quad b = #b$
  $a^T dot b = #dot-product$
  $a times b = #cross-product$
  $abs(a) = #length quad abs(a times b) = #cross-length$
]
