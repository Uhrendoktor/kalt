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
#let laplacian = comp($(
  #field[0:3, 1:4]
  + #field[2:5, 1:4]
  + #field[1:4, 0:3]
  + #field[1:4, 2:5]
  - 4 * #field[1:4, 1:4]
) / 0.25$)

#set align(center)

$ T = #field $
#v(3mm)
$ nabla_h^2 T = #laplacian $
