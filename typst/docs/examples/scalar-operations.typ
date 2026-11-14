#import "../../lib.typ": comp

#let add = comp($12 + 3$)
#let subtract = comp($12 - 3$)
#let multiply = comp($12 * 3$)
#let divide = comp($12 / 3$)
#let power = calc.round(comp($3^4$), digits: 0)
#let natural-log = calc.round(comp($ln(8)$), digits: 3)
#let logarithm = calc.round(comp($log_2(8)$), digits: 3)
#let magnitude = comp($abs(-3 - 4i)$)
#let rounded-up = comp($ceil(2.2)$)
#let rounded-down = comp($floor(2.8)$)
#let bit-and = comp($6 ∧ 3$)
#let bit-or = comp($6 ∨ 3$)
#let bit-xor = comp($6 ⊕ 3$)

#grid(
  columns: (1fr, 1fr, 1fr),
  gutter: 4mm,
  $12 + 3 = #add$,
  $12 - 3 = #subtract$,
  $12 * 3 = #multiply$,
  $12 / 3 = #divide$,
  $3^4 = #power$,
  $ln(8) = #natural-log$,
  $log_2(8) = #logarithm$,
  $abs(-3 - 4i) = #magnitude$,
  $ceil(2.2) = #rounded-up$,
  $floor(2.8) = #rounded-down$,
  $6 ∧ 3 = #bit-and$,
  $6 ∨ 3 = #bit-or$,
  $6 ⊕ 3 = #bit-xor$,
)
