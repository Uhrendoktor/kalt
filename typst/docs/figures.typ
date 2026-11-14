#import "../lib.typ": comp, map

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
#let north = comp($#field[0:3, 1:4]$)
#let south = comp($#field[2:5, 1:4]$)
#let west = comp($#field[1:4, 0:3]$)
#let east = comp($#field[1:4, 2:5]$)
#let center = comp($#field[1:4, 1:4]$)

#let laplacian = map(
  (n, s, w, e, c, _x, _y) => (n + s + w + e - 4 * c) / 0.25,
  north, south, west, east, center,
)

#set align(center)

$ T = #field $
#v(3mm)
$ nabla_h^2 T = #laplacian $
