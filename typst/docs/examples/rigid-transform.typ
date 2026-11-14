#import "../../lib.typ": comp

#let a = $mat(1, 2; 3, 4)$
#let b = $mat(5, 6; 7, 8)$
#let sum = comp($#a + #b$)
#let product = comp($#a dot #b$)

#align(center)[
  $A = #a quad B = #b$
  $A + B = #sum$
  $A dot B = #product$
]
