#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)

#let voltage = $100 + 0i$
#let impedance = $3 + 4i$
#let current = comp($#voltage / #impedance$)
#let conjugate = comp($#current^*$)
#let power = comp($#voltage * #conjugate$)

#grid(
  columns: (1fr, 1fr),
  gutter: 4mm,
  $V = #voltage " V"$,
  $Z = #impedance Ω$,
  $I = V / Z = #current " A"$,
  $I^* = #conjugate " A"$,
  $S = V I^* = #power " VA"$,
  $abs(S) = #comp($abs(#power)$) " VA"$,
)
