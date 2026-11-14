#import "../../lib.typ": comp

#set page(width: 104mm, height: auto, margin: 6mm)
#set text(size: 8.5pt)

#let position = $vec(0.4, 0.2, 0.1)$
#let force = $vec(100, -40, 25)$
#let axis = $vec(0, 0, 1)$
#let moment = comp($#position times #force$)
#let axial-force = comp($#axis^T dot #force$)
#let moment-size = calc.round(comp($abs(#moment)$), digits: 2)

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

#align(center)[#text(size: 15pt, weight: "bold", fill: ink)[FORCE & MOMENT]]
#align(center)[#text(size: 7pt, fill: muted)[3D STATICS · CROSS PRODUCT · DOT PRODUCT · VECTOR NORM]]
#v(7pt)

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 5pt,
  panel("POSITION · m", $r = #position$),
  panel("FORCE · N", $F = #force$),
  panel("AXIS", $e_z = #axis$),
)
#v(6pt)
#panel("MOMENT ABOUT THE ORIGIN", [
  #align(center)[$M = r times F = #moment " N" dot "m"$]
  #text(size: 7pt, fill: muted)[The cross product returns a three-component torque vector]
])
#v(5pt)
#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("AXIAL FORCE", [
    #text(size: 15pt, weight: "bold")[$e_z^T dot F = #axial-force " N"$]
    #text(size: 7pt, fill: muted)[Transpose and dot product]
  ]),
  panel("MOMENT MAGNITUDE", [
    #text(size: 15pt, weight: "bold")[$abs(M) = #moment-size " N" dot "m"$]
    #text(size: 7pt, fill: muted)[Vector norm]
  ]),
)
