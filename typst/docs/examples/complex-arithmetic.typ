#import "@preview/kalt:0.1.1": comp

#set page(width: 40mm, height: 33mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

#let voltage = $100 + 0i$
#let impedance = $3 + 4i$
#let current = comp($#voltage / #impedance$)
#let conjugate = comp($#current^*$)
#let power = comp($#voltage * #conjugate$)

$ V = #voltage " V" $

$ Z = #impedance Ω $

$ I = V / Z = #current " A" $

$ I^* = #conjugate " A" $

$ S = V I^* = #power " VA" $

$ abs(S) = #comp($abs(#power)$) " VA" $
