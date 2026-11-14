#import "../lib.typ": comp, map, reduce, any, all

#set page(width: 190mm, height: 50mm, margin: 6mm)
#set text(size: 17pt)
#set par(leading: 0.9em)

#let figure = sys.inputs.at("figure", default: "matrix-input")

#let fahrenheit = $mat(68, 77; 86, 95)$
#let celsius = map((value, x, y) => (value - 32) * 5 / 9, fahrenheit)
#let average = reduce((sum, value) => sum + comp(value), 0, celsius) / 4
#let has-hot-reading = any(value => comp(value) >= 30, celsius)
#let all-above-freezing = all(value => comp(value) > 0, celsius)

#align(center)[
  #if figure == "matrix-input" {
    $ A = mat(1, 2; 3, 4) quad B = mat(5, 6; 7, 8) $
  } else if figure == "matrix-expansion" {
    $ (A B)_(1, 1) &= 1 dot 5 + 2 dot 7 = 19 \\
      (A B)_(1, 2) &= 1 dot 6 + 2 dot 8 = 22 \\
      (A B)_(2, 1) &= 3 dot 5 + 4 dot 7 = 43 \\
      (A B)_(2, 2) &= 3 dot 6 + 4 dot 8 = 50 $
  } else if figure == "matrix-result" {
    $ A dot B
      &= comp(mat(1, 2; 3, 4) dot mat(5, 6; 7, 8)) $
  } else if figure == "vector-dot" {
    $ vec(1, 2, 3)^T dot vec(4, 5, 6)
      &= 1 dot 4 + 2 dot 5 + 3 dot 6 \\
      &= 4 + 10 + 18 \\
      &= comp(vec(1, 2, 3)^T dot vec(4, 5, 6)) $
  } else if figure == "vector-cross" {
    $ vec(1, 2, 3) × vec(4, 5, 6)
      &= vec(
        2 dot 6 - 3 dot 5,
        3 dot 4 - 1 dot 6,
        1 dot 5 - 2 dot 4
      ) \\
      &= comp(vec(1, 2, 3) × vec(4, 5, 6)) $
  } else if figure == "complex-product" {
    $ (2 + 3i)(4 - i)
      &= 8 - 2i + 12i - 3i^2 \\
      &= comp((2 + 3i)(4 - i)) $
  } else if figure == "temperature-map" {
    $ F = mat(68, 77; 86, 95) quad
      C = #celsius $
  } else if figure == "temperature-average" {
    $ overline(C) = (20 + 25 + 30 + 35) / 4 = #average quad °C $
  } else if figure == "temperature-predicates" {
    #text(size: 17pt)[At least one reading is 30 °C or higher: #has-hot-reading]
    #v(3mm)
    #text(size: 17pt)[Every reading is above 0 °C: #all-above-freezing]
  }
]
