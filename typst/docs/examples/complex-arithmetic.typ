#import "../../lib.typ": comp

#let voltage = $120 + 0i$
#let impedance = $30 + 40i$
#let current = comp($#voltage / #impedance$)
#let conjugate = comp($#current^*$)
#let power = comp($#voltage * #conjugate$)
#let magnitude = comp($abs(#power)$)

#align(center)[
  $V = #voltage " V" quad Z = #impedance Ω$
  $I = V / Z = #current " A"$
  $I^* = #conjugate " A"$
  $S = V I^* = #power " VA"$
  $abs(S) = #magnitude " VA"$
]
