#import "../../lib.typ": comp

#let a = $mat(1, 2; 3, 4)$
#let b = $mat(5, 6; 7, 8)$
#let add = comp($#a + #b$)
#let subtract = comp($#a - #b$)
#let multiply = comp($#a * #b$)
#let divide = comp($#b / #a$)
#let product = comp($#a dot #b$)
#let transpose = comp($#a^T$)
#let square = comp($#a^2$)

#grid(
  columns: (1fr, 1fr),
  gutter: 4mm,
  $A = #a quad B = #b$,
  $A^T = #transpose$,
  $A + B = #add$,
  $A - B = #subtract$,
  $A * B = #multiply$,
  $B / A = #divide$,
  $A dot B = #product$,
  $A^2 = #square$,
)
