#import "../../lib.typ": comp

#let add = comp($12 + 3$)
#let subtract = comp($12 - 3$)
#let multiply = comp($12 * 3$)
#let divide = comp($12 / 3$)
#let power = comp($3^4$)
#let natural-log = comp($ln(8)$)
#let logarithm = comp($log_2(8)$)
#let magnitude = comp($abs(-3 - 4i)$)
#let rounded-up = comp($ceil(2.2)$)
#let rounded-down = comp($floor(2.8)$)

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
)
