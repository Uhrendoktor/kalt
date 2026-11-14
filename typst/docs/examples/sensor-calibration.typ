#import "../../lib.typ": all, any, comp, map, reduce

#set page(width: 104mm, height: auto, margin: 6mm)
#set text(size: 8.5pt)

#let raw = $mat(
  1.02, 1.05, 0.98;
  0.99, 1.01, 1.04;
  1.03, 0.97, 1.00
)$

#let calibrated = map(
  (reading, x, y) => calc.round((reading - 1) * 100 - (x - y) * 0.5, digits: 1),
  raw,
)
#let total = reduce(
  (sum, value) => sum + comp(value),
  0,
  calibrated,
)
#let mean = total / 9
#let has_outlier = any(
  value => comp(value) > 4 or comp(value) < -4,
  calibrated,
)
#let all_in_range = all(
  value => comp(value) >= -5 and comp(value) <= 5,
  calibrated,
)

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

#align(center)[#text(size: 15pt, weight: "bold", fill: ink)[SENSOR CALIBRATION]]
#align(center)[#text(size: 7pt, fill: muted)[COORDINATE-AWARE CORRECTION · QUALITY GATES]]
#v(7pt)

#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("RAW READINGS", $R = #raw$),
  panel("CALIBRATED · %", $C = #calibrated$),
)
#v(6pt)
#panel("REDUCE · MEAN CORRECTION", [
  #align(center)[$C_"mean" = #mean %$]
  #text(size: 7pt, fill: muted)[Nine cells folded into one scalar]
])
#v(5pt)
#grid(
  columns: (1fr, 1fr),
  gutter: 6pt,
  panel("ANY · OUTSIDE ±4%", [
    #text(size: 13pt, weight: "bold", fill: ink)[#if has_outlier { "TRUE" } else { "FALSE" }]
    #text(size: 7pt, fill: muted)[At least one outlier]
  ]),
  panel("ALL · INSIDE ±5%", [
    #text(size: 13pt, weight: "bold", fill: ink)[#if all_in_range { "TRUE" } else { "FALSE" }]
    #text(size: 7pt, fill: muted)[Every sensor is in range]
  ]),
)
