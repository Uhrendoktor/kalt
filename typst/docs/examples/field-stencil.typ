#import "../../lib.typ": comp

#let matrix = $mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9
)$
#let center = comp($#matrix [1, 1]$)
#let middle_row = comp($#matrix [1, :]$)

#align(center)[
  $A = #matrix$
  $A_(1, 1) = #center$
  $A_(1, :) = #middle_row$
]
