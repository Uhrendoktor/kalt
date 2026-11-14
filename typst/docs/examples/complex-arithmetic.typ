#import "@preview/kalt:0.1.1": comp
#set page(width: 40mm, height: 25mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)
#let voltage = $100 + 0i$
#let impedance = $3 + 4i$
$ V &= #voltage " V" \
  Z &= #impedance Ω \
  I &= V / Z = #comp($#voltage / #impedance$) " A" \
  I^* &= #comp($(#voltage / #impedance)^*$) " A" \
  S &= V I^* = #comp($#voltage * (#voltage / #impedance)^*$) " VA" \
  abs(S) &= #comp($abs(#voltage * (#voltage / #impedance)^*)$) " VA" $
