#import "@preview/kalt:0.1.1": comp

#set page(width: 72mm, height: 37.5mm, margin: 1mm)
#set text(size: 8pt)
#set par(spacing: 1mm)

$ (2 + 3i)^2 = #comp($(2 + 3i)^2$) $

$ ((2 + 3 * (4^2 - 5)) / (6 - 2)) + log_2(2^3 + 8) - ln(e^2) = #comp($((2 + 3 * (4^2 - 5)) / (6 - 2)) + log_2(2^3 + 8) - ln(e^2)$) $

$ ((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2 = #comp($((12 + 3 * 4) > (2^3 + 2)) ∧ 6 ⊕ 3 ∨ 2$) $
