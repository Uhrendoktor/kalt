#import "../../lib.typ": comp

#let voltage = $120 + 0i$
#let impedance = $30 + 40i$
#let current = comp($#voltage / #impedance$)
#let power = comp($#voltage * #current^*$)
#let apparent = comp($abs(#power)$)

#align(center)[
  $V = #voltage " V" quad Z = #impedance Ω$
  $I = V / Z = #current " A"$
  $S = V I^* = #power " VA"$
  $abs(S) = #apparent " VA"$
]
