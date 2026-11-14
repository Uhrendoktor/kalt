#import "../lib.typ": comp

#set page(
  width: 190mm,
  height: auto,
  margin: (x: 6mm, y: 2mm),
)
#set text(size: 16pt)

#let field = $mat(
  10, 12, 15, 14, 11;
  13, 20, 25, 22, 15;
  14, 24, 35, 28, 16;
  12, 19, 27, 23, 14;
  9, 11, 16, 15, 10
)$
#let h2 = 0.25
#let north = $[0:3, 1:4]$
#let south = $[2:5, 1:4]$
#let west = $[1:4, 0:3]$
#let east = $[1:4, 2:5]$
#let center = $[1:4, 1:4]$

#let laplacian = comp($(
  #field#north + #field#south + #field#west + #field#east
  - 4 * #field#center
) / #h2$)

#align(center)[
  $ T = #field $
  #v(3mm)
  $ nabla_h^2 T = #laplacian $
]
