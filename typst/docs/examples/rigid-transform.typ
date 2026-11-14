#import "../../lib.typ": comp

#set page(width: 104mm, height: auto, margin: 6mm)
#set text(size: 8.5pt)

#let transform = $mat(
  0, -1, 5;
  1, 0, -2;
  0, 0, 1
)$
#let point = $vec(3, 2, 1)$
#let moved = comp($#transform dot #point$)
#let x = comp($#moved [0, 0]$)
#let y = comp($#moved [1, 0]$)
#let homogeneous = comp($#moved [2, 0]$)

#let ink = rgb("#182230")
#let muted = rgb("#667085")
#let panel(title, body) = block(
  width: 100%,
  inset: 8pt,
  radius: 5pt,
  fill: rgb("#f5f7fa"),
  stroke: 0.6pt + rgb("#e1e7ef"),
)[
  #text(size: 7pt, weight: "bold", fill: muted)[#title]
  #v(3pt)
  #align(center)[#body]
]

#align(center)[#text(size: 15pt, weight: "bold", fill: ink)[HOMOGENEOUS TRANSFORM]]
#align(center)[#text(size: 7pt, fill: muted)[2D POINT · 90° ROTATION · TRANSLATION]]
#v(7pt)

#panel("TRANSFORM MATRIX", $ T = #transform $)
#v(5pt)
#panel("INPUT POINT · HOMOGENEOUS COORDINATES", $ p = #point $)
#v(5pt)
#align(center)[
  #text(size: 7pt, fill: muted)[ONE MATRIX PRODUCT]
  #text(size: 9pt, weight: "bold", fill: ink)[$p' = T dot p$]
  #text(size: 7pt, fill: muted)[ROTATE COUNTER-CLOCKWISE, THEN SHIFT BY (5, −2)]
]
#v(5pt)
#panel("TRANSFORMED POINT", $ p' = #moved $)
#v(5pt)
#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 5pt,
  panel("x′", $ #x $),
  panel("y′", $ #y $),
  panel("w′", $ #homogeneous $),
)
