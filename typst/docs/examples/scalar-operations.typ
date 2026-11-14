#import "../../lib.typ": comp

#set page(width: 100mm, height: auto, margin: 3mm)
#set text(size: 8pt)

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 4mm,
  $12 + 3 = #comp($12 + 3$)$,
  $12 - 3 = #comp($12 - 3$)$,
  $12 * 3 = #comp($12 * 3$)$,
  $12 / 3 = #comp($12 / 3$)$,
  $3^4 = #comp($3^4$)$,
  $ln(8) = #calc.round(comp($ln(8)$), digits: 3)$,
  $log_2(8) = #calc.round(comp($log_2(8)$), digits: 3)$,
  $abs(-3 - 4i) = #comp($abs(-3 - 4i)$)$,
  $ceil(2.2) = #comp($ceil(2.2)$)$,
  $floor(2.8) = #comp($floor(2.8)$)$,
  $6 ∧ 3 = #comp($6 ∧ 3$)$,
  $6 ∨ 3 = #comp($6 ∨ 3$)$,
  $6 ⊕ 3 = #comp($6 ⊕ 3$)$,
)
