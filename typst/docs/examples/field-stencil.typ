#import "../../lib.typ": comp

#set page(width: 104mm, height: auto, margin: 6mm)
#set text(size: 8.5pt)

#let field = $mat(
  10, 12, 15, 14, 11;
  13, 20, 25, 22, 15;
  14, 24, 35, 28, 16;
  12, 19, 27, 23, 14;
  9, 11, 16, 15, 10
)$

#let north = comp($#field [0:3, 1:4]$)
#let south = comp($#field [2:5, 1:4]$)
#let west = comp($#field [1:4, 0:3]$)
#let east = comp($#field [1:4, 2:5]$)
#let middle = comp($#field [1:4, 1:4]$)
#let laplacian = comp($4 * (
  #north + #south + #west + #east
  - 4 * #middle
)$)

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

#align(center)[#text(size: 15pt, weight: "bold", fill: ink)[FIELD DIFFERENCES]]
#align(center)[#text(size: 7pt, fill: muted)[INDEXED WINDOWS · FIVE-POINT LAPLACIAN]]
#v(7pt)

#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("SAMPLED FIELD · 5 × 5", $ T = #field $),
  panel("STENCIL RESULT · 3 × 3", $ nabla_h^2 T = #laplacian $),
)

#v(6pt)
#panel("THE FIVE ALIGNED 3 × 3 SLICES", [
  #grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 4pt,
    $N = #north$,
    $S = #south$,
    $W = #west$,
    $E = #east$,
    $C = #middle$,
    align(center)[$K = mat(0, 1, 0; 1, -4, 1; 0, 1, 0)$],
  )
])
#v(5pt)
#align(center)[
  #text(size: 7pt, fill: muted)[h² = 0.25  ·  element-wise arithmetic  ·  no callback]
]
