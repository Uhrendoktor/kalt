#import "../../lib.typ": comp

#set page(width: 104mm, height: auto, margin: 6mm)
#set text(size: 8.5pt)

#let voltage = $120 + 0i$
#let impedance = $30 + 40i$
#let current = comp($#voltage / #impedance$)
#let current_conjugate = comp($#current^*$)
#let apparent = comp($#voltage * #current_conjugate$)
#let active = comp($Re(#apparent)$)
#let reactive = comp($Im(#apparent)$)
#let apparent_magnitude = comp($abs(#apparent)$)
#let power_factor = active / apparent_magnitude

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

#align(center)[
  #text(size: 15pt, weight: "bold", fill: ink)[AC POWER ANALYSIS]
  #text(size: 7pt, fill: muted)[SERIES R–L LOAD · COMPLEX ARITHMETIC]
]
#v(7pt)

#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("SOURCE VOLTAGE", $V = #voltage " V"$),
  panel("IMPEDANCE", $Z = #impedance Ω$),
)
#v(5pt)
#panel("OHM’S LAW · COMPLEX CURRENT", [
  #align(center)[$I = V / Z = #current " A"$]
  #text(size: 7pt, fill: muted)[Current is a complex phasor]
])
#v(5pt)
#panel("COMPLEX POWER · S = V I*", [
  #align(center)[$S = #apparent " VA"$]
  #grid(
    columns: (1fr, 1fr),
    gutter: 4pt,
    panel("ACTIVE · P", $ #active " W"$),
    panel("REACTIVE · Q", $ #reactive " var"$),
  )
])
#v(5pt)
#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("APPARENT POWER", $|S| = #apparent_magnitude " VA"$),
  panel("POWER FACTOR", $PF = #power_factor$),
)
